use std::env;
use std::path::Path;

pub fn cd() {
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(e)     => e.to_string(),
    };

    let path = Path::new(&home);
    env::set_current_dir(&path).is_ok();
}
