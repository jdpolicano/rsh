use std:: {
    io::{stdout, stdin, Read}
};

use rsh::term_interface::{TermInterface};

fn main() {
    let mut stdin = stdin().lock();
    let stdout = stdout().lock();
    let mut interface = TermInterface::new(stdin, stdout, "> ");
    loop {
        let mut buf = String::new();
        if interface.read_input(&mut buf).unwrap() > 0 {  
            println!("{:?}", buf);
        }
    }
}
