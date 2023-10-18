// Purpose: Dimensions of the screen.
use libc::{c_int, winsize, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std:: {
    process::Command
};

/// Represents the dimensions of the screen.
///
/// This struct provides information about the number of rows and columns
/// available in the terminal or console window.
///
/// It can be used to query the screen size and adapt your program's
/// output to fit the available space.
pub struct Dimensions {
    /// The number of rows in the screen.
    pub rows: usize,
    
    /// The number of columns in the screen.
    pub cols: usize,
}

impl Dimensions {
    /// Creates a new `Dimensions` instance by querying the screen size.
    ///
    /// This method attempts to retrieve the screen size using the `ioctl`
    /// system call. If that fails, it falls back to using the `tput` command
    /// to obtain the dimensions.
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

    /// Creates an empty `Dimensions` instance with zero rows and columns.
    ///
    /// This can be useful as a placeholder or default value.
    pub fn empty() -> Dimensions {
        Dimensions {
            rows: 0,
            cols: 0,
        }
    }


    /// Attempts to obtain screen dimensions using the `ioctl` system call.
    ///
    /// This method fills the provided `winsize` struct with the screen's row
    /// and column counts using the `TIOCGWINSZ` ioctl operation.
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

    /// Attempts to obtain screen dimensions using the `tput` command.
    ///
    /// This method executes the `tput` command to retrieve the number of lines
    /// and columns in the terminal window.
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