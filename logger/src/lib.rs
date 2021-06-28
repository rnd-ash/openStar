use colour::{green, green_ln, grey_ln, red_ln, yellow_ln};


pub trait Loggable {
    fn to_log_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Logger{
    module_name: String
}

impl Logger {

    pub fn new(module: &str) -> Self {
        Self {
            module_name: module.to_owned()
        }
    }

    pub fn log_err(&self, s: String) {
        red_ln!("[{}] ERROR: {}", self.module_name, s)
    }

    pub fn log_warn(&self, s: String) {
        yellow_ln!("[{}] WARN : {}", self.module_name, s)
    }

    pub fn log_info(&self, s: String) {
        println!("[{}] INFO : {}", self.module_name, s)
    }

    pub fn log_success(&self, s: String) {
        green_ln!("[{}] ERROR: {}", self.module_name, s)
    }

    pub fn log_debug(&self, s: String) {
        grey_ln!("[{}] DEBUG: {}", self.module_name, s)
    }

    pub fn log_object<T: Loggable>(&self, v: &T) {
        self.log_debug(v.to_log_string())
    }
}

unsafe impl Send for Logger{}
unsafe impl Sync for Logger{}