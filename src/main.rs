use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd};
use rsh::environment::EnvBuilder;
use rsh::keyboard::KeyBoardReader;
use rsh::terminal::Terminal;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    if let Ok(builder) = EnvBuilder::new(stdin.as_raw_fd()) {
        let term_env = builder
            .enable_raw_mode()
            .set_vmin(0)
            .set_vtime(1)
            .set_env()
            .unwrap();

        let mut keyboard = KeyBoardReader::new(stdin);
        let mut terminal = Terminal::new(keyboard, stdout);
        terminal.run();
        term_env.restore().unwrap();
    }
}
