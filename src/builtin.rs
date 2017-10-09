use std::env;
use std::path::Path;
use std::process;

pub fn _break() {
    
}

pub fn cd() {
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(e)     => e.to_string(),
    };

    let path = Path::new(&home);
    env::set_current_dir(&path).is_ok();
}

pub fn exit() {
    process::exit(0);
}

pub fn export() {
    
}
