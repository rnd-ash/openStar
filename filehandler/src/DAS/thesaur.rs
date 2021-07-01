use std::{fs::File, io::{BufRead, Cursor, Read, Seek, SeekFrom}, path::Path, str};
use logger::Logger;
use core::{convert::TryInto, num};


pub struct Thesaurus {
    lang: String,
    data: Cursor<Vec<u8>>,
    logger: logger::Logger,
    string_db_start: usize,
    max_strings: usize
}

impl Thesaurus {
    pub fn new(dbz: &Path, lang: String) -> Option<Self> {
        let logger = Logger::new("Thesaurus");

        logger.log_debug(format!("Loading {:?} as thesaurus for language {}", dbz, &lang));

        if dbz.extension().unwrap().to_ascii_uppercase() != "DBZ" {
            logger.log_err(format!("{:?} is not a thesaurus database!", dbz));
            return None
        }

        let mut file = match File::open(dbz) {
            Ok(f) => f,
            Err(e) => {
                logger.log_err(format!("Could not open {:?}!: {:?} ", dbz, e));
                return None;
            }
        };

        let mut buffer: Vec<u8> = Vec::new();

        if let Err(e) = file.read_to_end(&mut buffer) {
            logger.log_err(format!("Could not read {:?}!: {:?} ", dbz, e));
            return None;
        }

        if buffer.len() == 0 {
            logger.log_err(format!("Thesaurus {:?} is empty!", dbz));
            return None;
        }

        let unk1: u32 = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let string_db_start: u32 = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        let unk2: u32 = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        let num_strings: u32 = u32::from_le_bytes(buffer[12..16].try_into().unwrap());
        logger.log_debug(format!("Unk1 database located at offset 0x{:08X}", unk1));
        logger.log_debug(format!("Unk2 database located at offset 0x{:08X}", unk2));
        logger.log_debug(format!("String database located at offset 0x{:08X}, contains {} strings", string_db_start, num_strings));

        return Some(Self {
            lang,
            data: Cursor::new(buffer),
            logger,
            string_db_start: string_db_start as usize,
            max_strings: num_strings as usize,
        })
    }

    pub fn get_string(&mut self, idx: usize, max_chars: Option<usize>) -> Option<String> {
        if idx-1 >= self.max_strings || idx == 0 {
            self.logger.log_err(format!("String index {} out of range!", idx));
            return None
        }

        self.data.seek(SeekFrom::Start((self.string_db_start + (idx-1) * 4) as u64)).ok()?;

        let mut buffer: [u8; 4] = [0; 4];
        self.data.read_exact(&mut buffer).ok()?;

        let str_pos_start = u32::from_le_bytes(buffer);
        //self.logger.log_debug(format!("String {} located at offset {:08X?}", idx, str_pos_start));

        self.data.seek(SeekFrom::Start(str_pos_start as u64)).ok()?;

        let mut str_buffer: Vec<u8> = Vec::new();
        self.data.read_until(0x00, &mut str_buffer).ok()?;

        if let Some(max) = max_chars {
            str_buffer.drain(max..);
        }

        Some(String::from_utf8_lossy(&str_buffer).to_string())
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    pub fn load_test() {
    use std::path::PathBuf;

        let path = PathBuf::from("/home/ashcon/Mercedes-Benz/DAS/Thesaur/000/thesaual.dbz");

        let mut thesaurus = Thesaurus::new(path.as_path(), "en_GB".into()).unwrap();

        for idx in 1..10 {
            match thesaurus.get_string(idx, None) {
                Some(s) => println!("String {} => {}", idx, s),
                None => println!("Error locating string {}", idx)
            }
        }
    }
}