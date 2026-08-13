use serialport::{SerialPortType, available_ports};

/// Helper function to enumerate all connected serial devices
/// Returns them in a Vector
/// Used when user provides '-e' '--enumerate' CLI flag
pub fn enumerate_serial_devices() -> Vec<String> {
    let mut serial_devices: Vec<String> = Vec::new();

    match available_ports() {
        Ok(serial_ports) => {
            if serial_ports.is_empty() {
                return serial_devices;
            }
            for port in serial_ports {
                match port.port_type {
                    SerialPortType::UsbPort(_) => {
                        serial_devices.push(port.port_name);
                    }
                    _ => {}
                }
            }
        }
        Err(_) => {
            return serial_devices;
        }
    }

    return serial_devices;
}

/// Function to neatly print connected serial devices
/// Used when user provides '-e' '--enumerate' CLI flag
pub fn list_serial_devices(serial_devices: Vec<String>) {
    match serial_devices.len() {
        0 => {
            println!("No available serial devices...");
            return;
        }
        _ => {
            println!("Available Serial Devices:");
            for device in serial_devices {
                println!(">  {}", device)
            }
        }
    }
}
