use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageFormat};
use filehandler::PathOwner;
use iced::{Application, Settings, window::Icon};
use logger::Logger;
use nfd2::Response;
mod widgets;


mod launcher;
mod dialog;

pub static mut launcher_ok: bool = false;

const LAUNCHER_BYTES: &'static[u8] = include_bytes!("../assets/icon_high.png");

fn main() {
    #[cfg(unix)]
    std::env::set_var("WINIT_X11_SCALE_FACTOR", "1"); // Fix for X11 setup where DPI is detected incorrectly

    let mut settings = Settings::default();
    settings.window.resizable = false;
    settings.window.size = (800, 400);

    let master_logger = Logger::new("OpenStar init");

    if let Ok(img) = image::load_from_memory_with_format(LAUNCHER_BYTES, ImageFormat::Png) {
        settings.window.icon = Icon::from_rgba(img.clone().into_bytes(), img.width(), img.height()).ok()
    } else {
        master_logger.log_err("Could not load tray icon".into());
    }

    launcher::Launcher::run(settings);
}
