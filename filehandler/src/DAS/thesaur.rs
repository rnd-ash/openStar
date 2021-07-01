use std::{fs::File, io::{BufRead, Cursor, Read, Seek, SeekFrom}, path::Path};
use logger::Logger;
use core::{convert::TryInto};


/// Represents a theraurus that DAS uses in order to translate the UI
/// Between different languages
/// 
/// Essentially, it is just a lookup table the interface uses at run time to lookup
/// a specific String ID.
pub struct Thesaurus {
    lang: String,
    data: Cursor<Vec<u8>>,
    logger: logger::Logger,
    string_db_start: usize,
    max_strings: usize,
    is_utf16: bool
}

impl Thesaurus {

    /// Creates a new Thesaurus database.
    /// 
    /// ## Arguments
    /// * file - File path for the database. Should end in .DBZ or .DUZ
    /// * lang - Language ID. This is only used for logging purposes.
    pub fn new(file: &Path, lang: String) -> Option<Self> {
        let logger = Logger::new("Thesaurus");

        logger.log_debug(format!("Loading {:?} as thesaurus for language {}", file, &lang));

        // Test for DUZ or DBZ. DUZ is UTF16, and DBZ is UTF8
        let is_utf16 = if file.extension().unwrap().to_ascii_uppercase() == "DBZ" { // DBZ - UTF8
            false
        } else if file.extension().unwrap().to_ascii_uppercase() == "DUZ" { // DUZ - UTF16
            true
        } else {
            logger.log_err(format!("{:?} is not a thesaurus database!", file));
            return None
        };

        // Open the file
        let mut file = match File::open(file) {
            Ok(f) => f,
            Err(e) => {
                logger.log_err(format!("Could not open {:?}!: {:?} ", file, e));
                return None;
            }
        };

        // Byte buffer
        let mut buffer: Vec<u8> = Vec::new();

        // Read the file
        if let Err(e) = file.read_to_end(&mut buffer) {
            logger.log_err(format!("Could not read {:?}!: {:?} ", file, e));
            return None;
        }

        // Empty buffer?
        if buffer.len() == 0 {
            logger.log_err(format!("Thesaurus {:?} is empty!", file));
            return None;
        }

        let header: u32 = u32::from_le_bytes(buffer[0..4].try_into().unwrap()); // header byte
        let string_db_start: u32 = u32::from_le_bytes(buffer[4..8].try_into().unwrap()); // String database start offset in the file
        let unk2: u32 = u32::from_le_bytes(buffer[8..12].try_into().unwrap()); // Unknown - TODO
        let num_strings: u32 = u32::from_le_bytes(buffer[12..16].try_into().unwrap()); // Number of strings in the database

        // Check the file magic
        if header != 0x13421342 && !is_utf16 { // DBZ Header check
            logger.log_debug(format!("Thesaurus has a invalid header for DBZ: 0x{:08X?}", header));
            return None;
        } else if header != 0x43124312 && is_utf16 { // DUZ header check
            logger.log_debug(format!("Thesaurus has a invalid header for DUZ: 0x{:08X?}", header));
            return None;
        } else {
            logger.log_debug(format!("Thesaurus has a valid header: 0x{:08X?}", header));
        }
        logger.log_debug(format!("Unk2 database located at offset 0x{:08X}", unk2));
        logger.log_debug(format!("String database located at offset 0x{:08X}, contains {} strings", string_db_start, num_strings));

        return Some(Self {
            lang,
            data: Cursor::new(buffer),
            logger,
            string_db_start: string_db_start as usize,
            max_strings: num_strings as usize,
            is_utf16
        })
    }

    /// Gets a string with a specific index from the database, with an optional custom maximum size
    /// 
    /// ## Arguments
    /// * idx - The index of the string to get. THIS INDEX STARTS AT 1, NOT 0!!
    /// * max_chars - Optional, the maximum number of characters to grab from the string
    pub fn get_string(&mut self, idx: usize, max_chars: Option<usize>) -> Option<String> {
        // Check if string is out of range for the database
        if idx-1 >= self.max_strings || idx == 0 {
            self.logger.log_err(format!("String index {} out of range!", idx));
            return None
        }

        // Seek to the position in the file where the position of the string is located
        self.data.seek(SeekFrom::Start((self.string_db_start + (idx-1) * 4) as u64)).ok()?;

        let mut buffer: [u8; 4] = [0; 4];
        self.data.read_exact(&mut buffer).ok()?;

        let str_pos_start = u32::from_le_bytes(buffer); // String position in the file
        //self.logger.log_debug(format!("String {} located at offset {:08X?}", idx, str_pos_start));

        self.data.seek(SeekFrom::Start(str_pos_start as u64)).ok()?;


        if self.is_utf16 { // UTF 16 decoding is inefficient (For DUZ files)
            let mut str_buffer: Vec<u16> = Vec::new();
            // Slower - TODO Speed up UTF16 reading
            let mut tmp: u16 = 0;
            let mut buffer: [u8; 2] = [0; 2];
            loop {
                if self.data.read(&mut buffer).is_ok() {
                    tmp = (buffer[1] as u16) << 8 as u16 | (buffer[0] as u16);
                    if tmp == 0 {
                        break // End of string
                    } else {
                        str_buffer.push(tmp);
                    }
                } else {
                    break; // EOF?
                }
            }

            if let Some(max) = max_chars {
                str_buffer.drain(max..);
            }
            Some(String::from_utf16_lossy(&str_buffer).to_string())
        } else { // Use quick decoding for UTF8 (DBZ files)
            let mut str_buffer: Vec<u8> = Vec::new();
            self.data.read_until(0x00, &mut str_buffer).ok()?;

            if let Some(max) = max_chars {
                str_buffer.drain(max..);
            }
            Some(String::from_utf8_lossy(&str_buffer).to_string())
        }
    }
}