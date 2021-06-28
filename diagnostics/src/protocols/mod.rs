pub mod uds;
pub mod kwp2000;

#[derive(Debug)]
pub enum ProtocolError {
    ECUError(u8),
    ServerError(String),
    DeviceError(hardware::HardwareError)
}

impl Into<ProtocolError> for hardware::HardwareError {
    fn into(self) -> ProtocolError {
        ProtocolError::DeviceError(self)
    }
}

#[derive(Debug, Clone)]
pub enum DTCState {
    None,
    Stored,
    Pending,
    Active
}

#[derive(Debug, Clone)]
pub struct DTC {
    code: String,
    state: DTCState,
    mil_on: bool
}

pub type ProtocolResult<T> = std::result::Result<T, ProtocolError>;

pub trait GenericProtocolServer {
    fn send_command_with_response(&mut self, send: &[u8]) -> ProtocolResult<Vec<u8>>;

    fn send_command(&mut self, send: &[u8]) -> ProtocolResult<()>;

    fn read_dtcs(&mut self) -> ProtocolResult<Vec<DTC>>;

    
}