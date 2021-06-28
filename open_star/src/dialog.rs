use native_dialog::{MessageDialog, MessageType};

/// Shows an error in the event of an unrecoverable error, and the application must exit as a result
pub fn fatal_error(msg: &str) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("OpenStar fatal error")
        .set_text(msg)
        .show_alert()
        .unwrap();
}

/// Shows a warning message. The user has the option to acknowledge or cancel the error.
/// If the user declines, the return value is false, else the return value is true
pub fn warning(msg: &str) -> bool {
    MessageDialog::new()
        .set_type(MessageType::Warning)
        .set_title("OpenStar warning")
        .set_text(msg)
        .show_confirm()
        .unwrap()
}

