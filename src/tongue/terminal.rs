use std::{str, mem};

use libc;

use tongue::util;

const GET_CURSOR_POSITION: &str = "\x1b[6n";

pub struct CursorPosition {
    pub x: usize,
    pub y: usize,
}

pub fn get_cursor_position() -> CursorPosition {
    let mut buf = [0; 32];
    util::stdout(GET_CURSOR_POSITION);
    util::stdin(&mut buf);
    let buf = str::from_utf8(&buf[2 .. 32]).unwrap();
    let index_end = buf.find('R').unwrap();
    let index_middle = buf.find(';').unwrap();

    CursorPosition {
        y: buf.get(0 .. index_middle)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        x: buf.get(index_middle+1 .. index_end)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    }
}

pub fn tcgetattr() -> libc::termios {
    let mut term: libc::termios;
    unsafe {
        term = mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut term);
    }
    term
}

pub fn tcsetattr(mut term: libc::termios) {
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut term);
    }
}


pub fn enable_raw_mode() {
    unsafe {
        let mut term = tcgetattr();
        term.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        term.c_oflag &= !(libc::OPOST);
        term.c_cflag |= (libc::CS8);
        term.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        //term.c_cc[libc::VMIN] = 0;
        //term.c_cc[libc::VTIME] = 1;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut term);
    }
}
