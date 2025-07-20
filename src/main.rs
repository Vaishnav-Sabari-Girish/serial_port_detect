use serialport;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = serialport::available_ports()?;
    
    for port in ports {
        // Only show USB ports
        if let serialport::SerialPortType::UsbPort(ref info) = port.port_type {
            println!("Port: {}", port.port_name);
            println!("Type: {:?}", port.port_type);
            println!("  USB VID: {:04x}, PID: {:04x}", info.vid, info.pid);
            if let Some(manufacturer) = &info.manufacturer {
                println!("  Manufacturer: {}", manufacturer);
            }
            if let Some(product) = &info.product {
                println!("  Product: {}", product);
            }
            println!();
        }
    }
    Ok(())
}
