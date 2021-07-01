pub mod ini;

use std::path::PathBuf;

pub mod DAS;

static mut DAS_PATH: String = String::new();
static mut XEN_PATH: String = String::new();

pub fn set_das_path(p: PathBuf) {
    unsafe {
        DAS_PATH = p.to_str().unwrap().to_owned()
    }
}

pub fn set_xentry_path(p: PathBuf) {
    unsafe {
        XEN_PATH = p.to_str().unwrap().to_owned()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PathOwner {
    DAS,
    Xentry
}

pub fn create_local_path(owner: PathOwner, full_path: &str) -> PathBuf {
    let relative_path_split: Vec<&str> = match owner {
        PathOwner::DAS => full_path.split("DAS\\").collect(),
        PathOwner::Xentry => full_path.split("Xentry\\").collect(),
    };

    #[cfg(windows)]
    let path = relative_path_split[1].to_owned();
    #[cfg(unix)]
    let path = relative_path_split[1].replace("\\", "/").to_owned();

    unsafe {
        match owner {
            PathOwner::DAS => PathBuf::from(&DAS_PATH).join(path),
            PathOwner::Xentry => PathBuf::from(&XEN_PATH).join(path)
        }
    }
}

#[cfg(test)]
pub mod path_test {
    use super::*;

    #[test]
    pub fn test_unix_path_das() {
        let full_path = r"C:\Program Files (x86)\Mercedes-Benz\DAS\thesaur\000\thesaual.dbz";
        set_das_path(PathBuf::from("/home/user/Mercedes-Benz/DAS"));
        assert_eq!("/home/user/Mercedes-Benz/DAS/thesaur/000/thesaual.dbz", create_local_path(PathOwner::DAS, full_path).to_str().unwrap())
    }
}