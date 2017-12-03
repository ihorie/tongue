use std::io::{self, Write, BufWriter, Read};

pub fn stdin(mut buffer: &mut [u8]) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read(&mut buffer);
}

pub fn stdout(s: &str) {
    let stdout = io::stdout();
    let mut buf_writer = BufWriter::new(stdout.lock());
    buf_writer.write(s.as_bytes());
    buf_writer.flush();
}
