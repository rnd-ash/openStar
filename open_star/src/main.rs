use std::path::{Path, PathBuf};

use iced::{Settings, image};
use nfd2::Response;


mod launcher;
mod dialog;

pub static mut launcher_ok: bool = false;

const LAUNCHER_BYTES: &'static[u8] = include_bytes!("../assets/icon.png");

fn main() {
    // Before we do anything, we have to ask the user to set the Mercedes-Benz folder
    // for their existing DAS/Xentry install. I AM NOT RE-DISTRIBUTING THOSE FILES.

    #[cfg(Windows)]
    let default_path = Some(r"C:\Program Files (x86)\");
    #[cfg(unix)]
    let default_path = None;

    let mut valid_path: Option<PathBuf> = None;

    let master_logger = logger::Logger::new("OpenStar");

    match nfd2::open_pick_folder(default_path).unwrap() {
        Response::Okay(p) => valid_path = Some(p),
        _ => {
            master_logger.log_err("No path was selected!".into());
            dialog::fatal_error("No path was selected!\nOpenStar will now exit.");
            return;
        }
    }

    master_logger.log_debug(format!("Selected path is {:?}", &valid_path));

    let das_path = valid_path.clone().unwrap().join("DAS");
    let xen_path = valid_path.clone().unwrap().join("Xentry"); 

    if das_path.exists() {
        master_logger.log_debug(format!("DAS install located at {:?}", das_path));
    } else {
        master_logger.log_err("Path contains no DAS install!".into());
        dialog::fatal_error("The provided folder was invalid (DAS missing)!\nOpenStar will now exit.");
        return;
    }
    if xen_path.exists() {
        master_logger.log_debug(format!("Xentry install located at {:?}", xen_path));
    } else {
        master_logger.log_err("Path contains no Xentry install!".into());
        dialog::fatal_error("The provided folder was invalid (Xentry missing)!\nOpenStar will now exit.");
        return;
    }

}
