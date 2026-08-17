use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use meshtastic::api::StreamApi;
use meshtastic::utils;
use tokio::sync::Mutex;
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

    /// Function to initialize a connection with a specified serial port
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

        let serial_stream = match utils::stream::build_serial_stream(
            port.unwrap().clone(),
            Some(baud),
            Some(false),
            Some(false),
        ) {
            Ok(s) => s,
            Err(e) => return Err(ConnectionError::ConnectionFailed(e.to_string())),
        };

        let stream_api = StreamApi::new();
        let (mut packet_receiver, stream_api) = stream_api.connect(serial_stream).await;

        let config_id = utils::generate_rand_id();
        let stream_api = match stream_api
            .configure(config_id)
            .await
            .map_err(|e| ConnectionError::ConnectionFailed(e.to_string()))
        {
            Ok(s) => s,
            Err(e) => return Err(ConnectionError::ConnectionFailed(e.to_string())),
        };

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

    /// Function to disconnect the established serial connection
    pub fn disconnect(&mut self) -> Result<(), ConnectionError> {
        match self.handler.take() {
            Some(handle) => {
                handle.abort();
                Ok(())
            }
            None => Err(ConnectionError::NotConnected),
        }
    }

    /// Function to check if the serial connection is up
    pub fn is_connected(&self) -> bool {
        self.handler
            .as_ref()
            .map(|h| !h.is_finished())
            .unwrap_or(false)
    }
}

static CONNECTION: OnceLock<Mutex<DeviceConnection>> = OnceLock::new();

/// The single device connection for the whole app.
pub fn global_connection() -> &'static Mutex<DeviceConnection> {
    CONNECTION.get_or_init(|| Mutex::new(DeviceConnection::new()))
}
