use serialport::{SerialPortType, available_ports};

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

pub fn list_serial_devices(serial_devices: Vec<String>) {
    println!("Available Serial Devices:");
    for device in serial_devices {
        println!(">  {}", device)
    }
}
