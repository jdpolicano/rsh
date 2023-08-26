use std:: {
    io::{self, Read, Write},
    os::fd::{AsRawFd, RawFd}
};

use rsh::term_env::EnvBuilder;

fn main() {
    let mut stdin = io::stdin();
    if let Ok(builder) = EnvBuilder::new(stdin.as_raw_fd()) {
        let term_env = builder.enable_raw_mode().set_env().unwrap();

        let mut buf = [0];
        let mut stdout = io::stdout().lock();

        while stdin.read(&mut buf).is_ok() && buf[0] != b'q' {
            println!("as byte {}\r", buf[0]); // printing needs
            println!("as char {}\r", buf[0] as char);
        }

        term_env.restore().unwrap();
    }
}
