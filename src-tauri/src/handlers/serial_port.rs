use std::{io::Error, time::Duration};

use serialport::{
    DataBits, FlowControl, Parity, SerialPort, SerialPortBuilder, SerialPortInfo, StopBits,
};

pub struct SerialPortBuilderCustom {
    /// The port name, usually the device path
    pub path: Option<String>,
    /// The baud rate in symbols-per-second
    pub baud_rate: Option<u32>,
    /// Number of bits used to represent a character sent on the line
    pub data_bits: Option<DataBits>,
    /// The type of signalling to use for controlling data transfer
    pub flow_control: Option<FlowControl>,
    /// The type of parity to use for error checking
    pub parity: Option<Parity>,
    /// Number of bits to use to signal the end of a character
    pub stop_bits: Option<StopBits>,
    /// Amount of time to wait to receive data before timing out
    pub timeout: Option<Duration>,
    /// The state to set DTR to when opening the device
    pub dtr_on_open: Option<bool>,
}

#[tauri::command]
pub fn get_ports_info() -> Result<Vec<SerialPortInfo>, String> {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                Err("Порты не найдены".to_string())
            } else {
                log::info!("Ports len: {}", ports.len());
                Ok(ports)
            }
        }
        Err(e) => Err(format!("Ошибка при получении портов: {}", e)),
    }
}

fn set_port(port_name: &String) -> SerialPortBuilder {
    serialport::new(port_name, 9600)
}

#[tauri::command]
pub fn settings_port(port_name: String, params: SerialPortBuilderCustom) -> Result<String, String> {
    // Начинаем с базового порта
    let mut port = set_port(&port_name);

    if let Some(baud_rate) = params.baud_rate {
        port = port.baud_rate(baud_rate);
    }
    if let Some(data_bits) = params.data_bits {
        port = port.data_bits(data_bits);
    }
    if let Some(flow_control) = params.flow_control {
        port = port.flow_control(flow_control);
    }
    if let Some(parity) = params.parity {
        port = port.parity(parity);
    }
    if let Some(stop_bits) = params.stop_bits {
        port = port.stop_bits(stop_bits);
    }
    if let Some(timeout) = params.timeout {
        port = port.timeout(timeout);
    }
    if let Some(dtr_on_open) = params.dtr_on_open {
        port = port.dtr_on_open(dtr_on_open);
    }

    Ok("Port settings updated successfully".to_string())
}
