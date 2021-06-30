use std::{collections::HashMap, path::PathBuf};

use configparser::ini::Ini;


pub struct IniFile{
    ini: Ini
}

impl IniFile {
    pub fn new(path: PathBuf) -> Option<Self> {
        let mut ini = Ini::new();
        ini.load(path).ok()?;
        Some(Self { ini })
    }

    pub fn find_value_section(&mut self, header: &str, key: &str) -> Option<String> {
        self.ini.get(header, key)
    }

    pub fn find_section(&mut self, header: &str) -> Option<Vec<(String, Option<String>)>> {
        let mut res = Vec::new();
        let header_map = self.ini.get_map()?;
        let section = header_map.get(&header.to_ascii_lowercase())?;

        for (k, v) in section.iter() {
            res.push((k.clone(), v.clone()))
        }
        Some(res)
    }
}

