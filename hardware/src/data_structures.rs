use std::{cmp::min, fmt::Debug};

pub trait HwDataFrame: Debug + Sync + Send + Sized + Clone + Default {
    fn set_data(&mut self, data: &[u8]);
    fn get_data(&self) -> &[u8];
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
}

#[derive(Debug, Clone, Default)]
pub struct HWCanFrame {
    id: u32,
    data: [u8; 8],
    dlc: u8,
    pub can_ext_addr: bool
}

impl HWCanFrame {
    pub fn new(id: u32, data: &[u8]) -> Self {
        let mut c = Self::default();
        c.set_data(data);
        c.set_id(id);
        c
    }
}

impl HwDataFrame for HWCanFrame {
    fn set_data(&mut self, data: &[u8]) {
        let max = min(data.len(), 8);
        self.dlc = max as u8;
        self.data[0..max].copy_from_slice(&data[0..max]);
    }

    fn get_data(&self) -> &[u8] {
        &self.data[0..self.dlc as usize]
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
        self.can_ext_addr = self.id > 0x7FF;
    }
}

impl logger::Loggable for HWCanFrame {
    fn to_log_string(&self) -> String {
        format!("CanFrame - ID: 0x{:04X}, Data: {:02X?}", self.id, &self.data[0..self.dlc as usize])
    }
}


#[derive(Debug, Clone, Default)]
pub struct HwIsoTpFrame {
    id: u32,
    can_ext_addr: bool,
    ext: bool,
    data: Vec<u8>
}

impl HwIsoTpFrame {
    pub fn new(id: u32, isotp_ext_addr: bool, data: &[u8]) -> Self {
        let mut c = Self::default();
        c.ext = isotp_ext_addr;
        c.set_data(data);
        c.set_id(id);
        c
    }
}

impl HwDataFrame for HwIsoTpFrame {
    fn set_data(&mut self, data: &[u8]) {
        self.data = data.to_vec()
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
        self.can_ext_addr = self.id > 0x7FF;
    }
}

impl logger::Loggable for HwIsoTpFrame {
    fn to_log_string(&self) -> String {
        format!("IsoTPFrame - ID: 0x{:04X}, Ext: {}, Data: {:02X?}", self.id, self.ext, self.data)
    }
}

#[derive(Debug, Clone)]
pub struct HwKwpFrame {
    id: u32,
    data: Vec<u8>
}


#[cfg(test)]
pub mod test {

    use logger::{Loggable, Logger};

    use super::*;

    #[test]
    pub fn test_log() {
        let logger = Logger::new("Hardware");
        let can = HWCanFrame::new(0x001C, &[0x00, 0x01, 0x02, 0x03]);
        logger.log_object(&can);
    }
}