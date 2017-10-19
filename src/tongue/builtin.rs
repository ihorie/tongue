use std::env;
use std::path::Path;
use std::process;

pub fn dot() {

}

pub fn alias(tokens: Vec<String>) {
    if let Some((command, options)) = tokens.split_first() {
        if options.is_empty() {
            
        } else {
            let v: Vec<&str> = options[0].split('=').collect();
            
//            aliases.insert(v[0], v[1]);
        }
    }
}

pub fn _break() {
    
}

pub fn cd(tokens: Vec<String>) {
    if let Some((command, options)) = tokens.split_first() {
        if options.is_empty() {
            let home = match env::var("HOME") {
                Ok(home) => home,
                Err(e)   => e.to_string(),
            };

            let path = Path::new(&home);
            env::set_current_dir(&path).is_ok();
        } else {
            env::set_current_dir(&options[0]).is_ok();
        }
    }
}

pub fn _continue() {
  
}

pub fn exec() {

}

pub fn exit() {
    process::exit(0);
}

pub fn export() {
    
}

pub fn _return() {

}
