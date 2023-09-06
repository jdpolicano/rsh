// Purpose: Dimensions of the screen.
use libc::{c_int, winsize, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std:: {
    process::Command
};

pub struct Dimensions {
    pub rows: usize,
    pub cols: usize,
}

impl Dimensions {
    pub fn new() -> Dimensions {
        let mut winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        Dimensions::get_winsize(&mut winsize);

        Dimensions {
            rows: winsize.ws_row as usize,
            cols: winsize.ws_col as usize,
        }
    }

    pub fn empty() -> Dimensions {
        Dimensions {
            rows: 0,
            cols: 0,
        }
    }


    pub fn get_winsize(winsize: &mut winsize) {
        // try ioctl first, if that fails, try tput.
        let ioctl_result = Dimensions::from_ioctl(winsize);
        if ioctl_result == -1 {
            Dimensions::from_tput(winsize);
        }
    }

    // try ioctl first, if that fails, try tput.
    pub fn from_ioctl(winsize: &mut winsize) -> c_int {
        unsafe {
            ioctl(STDOUT_FILENO, TIOCGWINSZ, winsize)
        }
    }

    pub fn from_tput(winsize: &mut winsize) {
        let output = Command::new("tput")
            .arg("lines")
            .output()
            .expect("failed to get window height");

        
        let mut rows = String::from_utf8_lossy(&output.stdout).into_owned();
        rows.pop(); // remove the newline character.

        winsize.ws_row = rows.parse::<u16>().unwrap();

        let output = Command::new("tput")
            .arg("cols")
            .output()
            .expect("failed to get window width");
        

        let mut cols = String::from_utf8_lossy(&output.stdout).into_owned();
        cols.pop(); // remove the newline character.

        winsize.ws_col = cols.parse::<u16>().unwrap();
    }

    pub fn resize (&mut self) {
        let mut winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        Dimensions::get_winsize(&mut winsize);

        self.rows = winsize.ws_row as usize;
        self.cols = winsize.ws_col as usize;
    }
}