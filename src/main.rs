use std::io::{self, Write};
use std::process::exit;

fn main() {
    tongue_main()
}

fn tongue_main() {
    loop {
        print!(" ¥ ");
	io::stdout().flush().unwrap();

        let mut buf = String::new();

        io::stdin().read_line(&mut buf)
	    .expect("Failed to read line");

        if "".eq(&buf) {
	    println!("");
	    exit(0);
	}
	 

        parse();
    }
}

fn parse() {
    eval();
}

fn eval() {
}