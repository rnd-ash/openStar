use std::fmt::{Debug, format};

use communication_apis::passthru;
use data_structures::HwDataFrame;
use j2534_rust::Loggable;
use logger::Logger;

pub mod data_structures;
mod communication_apis;

extern crate j2534_rust;

#[derive(Debug)]
pub enum HardwareError {
    HwApiError { code: u32, desc: String },
    IoError(std::io::Error),
    Other(String)
}

// Converts PassthruError into HardwareError to allow for the '?' operator (TryInto)
impl Into<HardwareError> for j2534_rust::PassthruError {
    fn into(self) -> HardwareError {
        HardwareError::HwApiError {
            code: self as u32,
            desc: self.to_string().into()
        }
    }
}

/// Enum representing the various communication protocols that can be established with the vehicle
/// as logical communication channels
#[derive(Debug, Copy, Clone)]
pub enum AdapterChannel {
    /// Canbus channel (ISO11898)
    Can,
    /// ISOTP channel (ISO15765)
    IsoTp,
    /// KWP over LIN channel (ISO14230)
    Kwp,
    /// OBD channel (ISO9141)
    Obd
}

/// Adapter buffer types
#[derive(Debug, Copy, Clone)]
pub enum AdapterBuffer {
    /// Input buffer (Vehicle to adapter)
    Input,
    /// Output buffer (Adapter to vehicle)
    Output,
    /// Both buffers
    Both
}

/// Filter types for the adapter
#[derive(Debug, Copy, Clone)]
pub enum AdapterFilter {
    /// Pass filter. Data will be allowed to be read if its ID matches the following logical expression:
    /// `mask & id == ID`
    Pass { mask: u32, id: u32 },
    /// Pass filter. Data will be allowed to be read if its ID matches the following logical expression:
    /// `mask & id != ID`
    Block{ mask: u32, id: u32 },
    /// Special filter for IsoTp. Acts like [AdapterFilter::Pass], but has an additional parameter for
    /// flow control.
    IsoTP{ mask: u32, id: u32, fc: u32}
}

/// IOCTL identifiers. Used for [AdapterHardware::channel_ioctl]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum IoctlIdentifier {
    /// ISO TP seperation time (MS)
    ISO15765_STMIN(u32),
    /// ISO TP block size
    ISO15765_BS(u32),
    /// ISO 9141 minimum ECU inter-byte time
    P1_MIN(u32),
    /// ISO 9141 maximum ECU inter-byte time
    P1_MAX(u32),
    /// ISO 9141 minimum ECU response time
    P2_MIN(u32),
    /// ISO 9141 maximum ECU response time
    P2_MAX(u32),
    /// ISO 9141 minimum ECU response time between response and next request
    P3_MIN(u32),
    /// ISO 9141 maximum ECU response time between response and next request
    P3_MAX(u32),
    /// ISO 9141 minimum tester inter-byte time for a request
    P4_MIN(u32),
    /// ISO 9141 maximum tester inter-byte time for a request
    P4_MAX(u32),
    /// ISO 9141 maximum time from the address byte end to synchronization pattern start
    W1(u32),
    /// ISO 9141 maximum time from the synchronization byte end to key byte 1 start
    W2(u32),
    /// ISO 9141 maximum time between key byte 1 and key byte 2
    W3(u32),
    /// ISO 9141 maximum time between key byte 2 and its inversion from the tester
    W4(u32),
    /// ISO 9141 minimum time before the tester begins retransmission of the address byte
    W5(u32),
    /// ISO 9141 bus idle time before starting a fast initialization sequence
    TIDLE(u32),
    /// ISO 9141 duration of the fast initialization low pulse
    TINL(u32),
    /// ISO 9141 duration of the fast initialization wake-up pulse
    TWUP(u32),
    /// ISO9141 parity (0 = No parity, 1 = Odd parity, 2 = Even parity)
    PARITY(u8)
}

/// Flags which are applied to a channel upon its creation
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum ChannelFlags {
    /// CAN Network uses 29bit addressing
    CAN_USE_29BIT_ADDR,
    /// ISOTP uses extended addressing
    ISOTP_USE_EXT_ADDR,
    /// LIN channel does not use checksum
    ISO9141_NO_CHECKSUM
}


/// Initialization type for LIN based communication channels
#[derive(Debug, Clone)]
pub enum LinInitType {
    /// Fast initialization
    FastInit { id: u32, data: Vec<u8> },
    /// Five baud init
    FiveBaudInit(Vec<u8>),
}

pub type HardwareResult<T> = std::result::Result<T, HardwareError>;

/// Dynamic trait for any adapter in order to communicate with a vehicle, such a passthru or D-PDU
pub trait AdapterHardware: Sized + Sync + Send + Debug + Clone {

    /// Attempts to open the device and load its associated library
    fn open_device(&mut self) -> HardwareResult<()>;

    /// Attempts to close the device, terminating any connections to the vehicle in the process
    fn close_device(&mut self) -> HardwareResult<()>;

    /// Reads the voltage of the vehicle by probing the VBATT bin on the OBD port
    fn read_voltage(&mut self) -> HardwareResult<f32>;
    /// Opens a logical communication link to the vehicle. On some APIs such as Passthru,
    /// opening up a CAN and ISOTP channel will fail as the channel types cannot co-exist.
    /// To avoid an open floodgate, the channel will block ALL traffic once opened. Use [AdapterHardware::add_channel_filter]
    /// in order to allow the receiving of channel data.
    ///
    /// ## Arguments
    /// * channel_type - The channel type to open with the vehicle.
    /// 
    /// ## Returns
    /// This function (If successful), will return a unique ID which can be used for adding or deleting filters.
    fn open_channel(&mut self, channel_type: AdapterChannel) -> HardwareResult<u32>;

    /// Attempts to close a communication channel
    /// 
    /// ## Arguments
    /// * id - The unique channel ID to close, provided by [AdapterHardware::open_channel]
    fn close_channel(&mut self, id: u32) -> HardwareResult<()>;

    /// Creates a filter on a specified channel
    /// 
    /// ## Arguments
    /// * channel_id - The ID of the channel to create the filter on
    /// * filter - The type of filter to configure
    /// * baud - The bus speed of the channel (In bps)
    /// * flags - A list of flags to be applied to the channel. For example, using extended CAN.
    /// 
    /// ## Returns
    /// * Returns a unique ID of the filter. This ID is unique to the channel.
    fn add_channel_filter(&mut self, channel_id: u32, filter: AdapterFilter, baud: u32, flags: &[ChannelFlags]) -> HardwareResult<u32>;

    /// Deletes a channel on a specified channel
    /// 
    /// ## Arguments
    /// * channel_id - The ID of the channel to delete the filter
    /// * filter_id - The ID of the filter to delete
    fn del_channel_filter(&mut self, channel_id: u32, filter_id: u32) -> HardwareResult<u32>;

    /// Attempts to clear a given channels buffer.
    /// 
    /// ## Arguments
    /// * channel_id - ID of the channel to clear
    /// * buffer - The type of buffer to clear
    fn clear_channel_buffer(&mut self, channel_id: u32, buffer: AdapterBuffer) -> HardwareResult<()>;


    // Reads data from the vehicle. The channel to receive data from is determined based on the Data type.
    /// 
    /// ## Arguments
    /// * max_read - The maximum amount of messages to receive from the vehicle.
    /// * timeout_ms - The read timeout. A value of 0 implies the adapter shall not wait for any additional data,
    ///     and instead just return whatever is in its channel FIFO queue.
    /// 
    /// ## Returns
    /// * An array of data messages read from the vehicle. The size of this array can be less than [max_read]
    fn read_data<T: HwDataFrame>(&mut self, max_read: usize, timeout_ms: u128) -> HardwareResult<Vec<T>>;

    /// Writes data to the vehicle. The channel to transmit data on is determined based on the Data type.
    /// 
    /// ## Arguments
    /// * input - An array of data to send to the vehicle
    /// * timeout_ms - The write timeout in ms. A value of 0 implies the data is sent to the adapter and then
    ///     the function returns immediately, without checking if the data was successfully sent.
    /// 
    /// ## Panics
    /// This function will panic if all the data in the [input] array is NOT of the same payload type
    fn write_data<T: HwDataFrame>(&mut self, input: &[T], timeout_ms: u128) -> HardwareResult<()>;

    /// Attempts to write a single message to the vehicle, then poll for a response from the logical channel.
    /// The channel's filters should be configured first before calling this function to avoid an open floodgate
    /// situation where any message is received as a response. This function is best used on an ISOTP, OBD, or KWP channel type.
    /// 
    /// ## Arguments
    /// * write - The message to write to the vehicle
    /// * write_timeout_ms - The write timeout. 0 implies the function returns immediately without checking if the message was sent!
    /// * read_timeout_ms - The read timeout. 0 implies no waiting for incoming messages. Just grab whatever is in the adapter FIFO buffer.
    /// 
    /// ## Returns
    /// If successful, this function will return a response message.
    fn read_and_write<T: HwDataFrame>(&mut self, write: T, write_timeout_ms: u128, read_timeout_ms: u128) -> HardwareResult<T> {
        self.write_data(&[write], write_timeout_ms)?;
        self.read_data(1, read_timeout_ms).map(|read: Vec<T>| read[0].clone())
    }

    /// Configures a channel with a IOCTL parameter
    /// 
    /// ## Arguments
    /// * channel_id - The ID of the channel to perform the IOCTL operation on
    /// * param - The IOCTL parameter to apply to the channel
    fn channel_set_ioctl(channel_id: u32, param: IoctlIdentifier) -> HardwareResult<()>;

    /// Reads a channel's IOCTL parameter
    /// 
    /// ## Arguments
    /// * channel_id - The ID of the channel to read the IOCTL parameter from
    /// * param - The IOCTL parameter to read from the channel. The value within will be set if this function succeeds
    fn channel_get_ioctl(channel_id: u32, param: &mut IoctlIdentifier) -> HardwareResult<()>;


    /// Performs a LIN based initialization of a LIN channel
    /// 
    /// ## Arguments
    /// * channel_id - The ID of the LIN channel to initialize
    /// * init_type - Mutable reference to the initialization type of the channel. If this function succeeds,
    ///     the data within this will be replaced by the response from the ECU.
    fn channel_lin_init(channel_id: u32, init_type: &mut LinInitType) -> HardwareResult<()>;

    /// Attempts to reset the Adapter by closing and opening it again (Turning it off and on again)
    fn reset_device(&mut self) -> HardwareResult<()> {
        self.close_device()?;
        self.open_device()
    }
}



#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HardwareAPI {
    None,
    Passthru,
    Pdu,
    Sd,
    Sim,
    #[cfg(unix)]
    SocketCAN
}

impl std::fmt::Display for HardwareAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareAPI::None => f.write_str("NULL"),
            HardwareAPI::Passthru => f.write_str("Passthru"),
            HardwareAPI::Pdu => f.write_str("D-PDU"),
            HardwareAPI::Sd => f.write_str("SDConnect"),
            HardwareAPI::Sim => f.write_str("Simulation"),
            #[cfg(unix)]
            HardwareAPI::SocketCAN => f.write_str("SocketCAN"),
        }
    }
}

impl Default for HardwareAPI {
    fn default() -> Self {
        HardwareAPI::None
    }
}

pub fn get_device_list(api: HardwareAPI) -> Vec<String> {
    let logger = Logger::new("Hardware");
    match api {
        HardwareAPI::Sim => vec!["OpenStar-Simulation".into()],
        HardwareAPI::Passthru => {
            logger.log_debug("Scanning for Passthru devices".into());
            match passthru::PassthruDevice::find_all() {
                Ok(ptd) => {
                    for d in &ptd {
                        logger.log_debug(format!("=> Found passthru device: {}", d.name));
                    }
                    ptd.iter().map(|s| s.name.clone()).collect()
                },
                Err(e) => {
                    logger.log_err(format!("=> Scanning for Passthru devices failed: {:?}", e));
                    Vec::new()
                }
            }
        }
        _ => Vec::new()
    }
}

pub fn open_device(name: &str, api: HardwareAPI) -> bool {
    let logger = Logger::new("Hardware");
    logger.log_debug(format!("Trying to open device '{}' using {} API", name, api));
    return true;
}