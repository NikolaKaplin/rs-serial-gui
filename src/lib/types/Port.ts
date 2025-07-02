export interface Port {
    port_name: string;
    port_type: SerialPortType;
}

type SerialPortType =
    | { type: 'UsbPort'; info: UsbPortInfo }
    | { type: 'PciPort' }
    | { type: 'BluetoothPort' }
    | { type: 'Unknown' };

type UsbPortInfo = {
    vid: number,
        /// Product ID
    pid: number,
        /// Serial number (arbitrary string)
    serial_number?: string,
        /// Manufacturer (arbitrary string)
    manufacturer?: string,
        /// Product name (arbitrary string)
    product?: string,
    /// The interface index of the USB serial port. This can be either the interface number of
    /// the communication interface (as is the case on Windows and Linux) or the data
    /// interface (as is the case on macOS), so you should recognize both interface numbers.
    interface?:  number,
}