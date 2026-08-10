use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use meshtastic::api::StreamApi;
use meshtastic::utils;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use crate::meshtastic::errors::ConnectionError;
use crate::meshtastic::message_handler::handle_from_radio_packet;
pub struct DeviceConnection {
    handler: Option<JoinHandle<()>>,
    connected: Arc<AtomicBool>,
}

impl DeviceConnection {
    pub fn new() -> Self {
        Self {
            handler: None,
            connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn connect(
        &mut self,
        port: Option<String>,
        baud: u32,
    ) -> Result<(), ConnectionError> {
        if self.is_connected() {
            return Err(ConnectionError::AlreadyConnected);
        }
        if let None = port {
            return Err(ConnectionError::PortNotFound("None".to_string()));
        }

        let serial_stream = utils::stream::build_serial_stream(
            port.unwrap().clone(),
            Some(baud),
            Some(false),
            Some(false),
        )?;

        let stream_api = StreamApi::new();
        let (mut packet_receiver, stream_api) = stream_api.connect(serial_stream).await;

        let config_id = utils::generate_rand_id();
        let stream_api = stream_api
            .configure(config_id)
            .await
            .map_err(|e| ConnectionError::ConnectionFailed(e.to_string()))?;

        self.connected.store(true, Ordering::SeqCst);
        let connected = self.connected.clone();

        let handler = tokio::spawn(async move {
            while let Some(from_radio_packet) = packet_receiver.recv().await {
                handle_from_radio_packet(from_radio_packet).await;
            }

            connected.store(false, Ordering::SeqCst);
            let _ = stream_api.disconnect().await;
        });

        self.handler = Some(handler);
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), ConnectionError> {
        match self.handler.take() {
            Some(handle) => {
                handle.abort();
                Ok(())
            }
            None => Err(ConnectionError::NotConnected),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.handler
            .as_ref()
            .map(|h| !h.is_finished())
            .unwrap_or(false)
    }

    pub fn connected_flag(&self) -> Arc<AtomicBool> {
        self.connected.clone()
    }
}

static RUNTIME: OnceLock<Runtime> = OnceLock::new();
static CONNECTION: OnceLock<Mutex<DeviceConnection>> = OnceLock::new();

/// The single Tokio runtime the whole app's async work runs on. Lives for
/// the lifetime of the program (OnceLock is never dropped until process exit),
/// so a spawned listener task keeps running no matter what UI screen is active.
pub fn global_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"))
}

/// The single device connection for the whole app.
pub fn global_connection() -> &'static Mutex<DeviceConnection> {
    CONNECTION.get_or_init(|| Mutex::new(DeviceConnection::new()))
}
