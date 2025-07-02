
use serialport::{ SerialPortInfo};


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
        Err(e) => Err(format!("Ошибка при получении портов: {}", e))
    }
}

