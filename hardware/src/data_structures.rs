use std::{cmp::min, fmt::Debug};

pub trait HwDataFrame: Debug + Sync + Send + Sized + Clone {
    fn set_data(&mut self, data: &[u8]);
    fn get_data(&self) -> &[u8];
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn get_name(&self) -> &str;

    fn to_log(&self) -> String {
        format!("{} ID: 0x{}, Data: {:02X?}", self.get_name(), self.get_id(), self.get_data())
    }
}

#[derive(Debug, Clone)]
pub struct HWCanFrame {
    id: u32,
    data: [u8; 8],
    dlc: u8,
    ext_addr: bool
}

impl HwDataFrame for HWCanFrame {
    fn set_data(&mut self, data: &[u8]) {
        let max = min(data.len(), 8);
        self.dlc = max as u8;
        self.data[0..max].copy_from_slice(&data[0..8]);
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_name(&self) -> &str {
        "CanFrame"
    }
}


#[derive(Debug, Clone)]
pub struct HwIsoTpFrame {
    id: u32,
    ext_addr: bool,
    ext: bool,
    data: Vec<u8>
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
        self.id = id
    }

    fn get_name(&self) -> &str {
        "IsoTpFrame"
    }
}

#[derive(Debug, Clone)]
pub struct HwKwpFrame {
    id: u32,
    data: Vec<u8>
}