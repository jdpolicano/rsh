use termios::{
    self, 
    Termios, 
    ECHO, 
    ICANON,
    ISIG,
    IXON,
    IEXTEN,
    ICRNL,
    OPOST,
    BRKINT,
    INPCK,
    ISTRIP,
    CS8,
    TCSAFLUSH, 
    tcsetattr
};

use std::os::fd::RawFd;
use std::io;

pub struct EnvBuilder {
    fd: RawFd,
    original_state: Termios,
    transform_state: Termios,
}


impl EnvBuilder {
    pub fn new(fd: RawFd) -> Result<EnvBuilder, io::Error> {
        let original_state = termios::Termios::from_fd(fd)?; // read in current state.
        let transform_state = original_state.clone(); // this will be edited and later restored to original state. 

        Ok(EnvBuilder {
            fd,
            original_state,
            transform_state
        })
    }

    pub fn echo(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_lflag |= ECHO;
        } else  {
            self.transform_state.c_lflag &= !ECHO;
        }
        self
    }

    pub fn i_canon(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_lflag |= ICANON;
        } else {
            self.transform_state.c_lflag &= !ICANON;
        }
        self
    }

    pub fn i_sig(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_lflag |= ISIG;
        } else {
            self.transform_state.c_lflag &= !ISIG;
        }
        self
    }

    pub fn i_exten(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_lflag |= IEXTEN;
        } else {
            self.transform_state.c_lflag &= !IEXTEN;
        }
        self
    }

    pub fn i_xon(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_iflag |= IXON;
        } else {
            self.transform_state.c_iflag &= !IXON;
        }
        self
    }

    pub fn i_crnl(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_iflag |= ICRNL;
        } else {
            self.transform_state.c_iflag &= !ICRNL;
        }
        self
    }
    

    pub fn o_post(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_oflag |= OPOST;
        } else {
            self.transform_state.c_oflag &= !OPOST;
        }
        self
    }

    pub fn brkint(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_oflag |= BRKINT;
        } else {
            self.transform_state.c_oflag &= !BRKINT;
        }
        self
    }

    pub fn inpck(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_oflag |= INPCK;
        } else {
            self.transform_state.c_oflag &= !INPCK;
        }
        self
    }

    pub fn i_strip(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_oflag |= ISTRIP;
        } else {
            self.transform_state.c_oflag &= !ISTRIP;
        }
        self
    }

    pub fn cs8(mut self, turn_on: bool) -> EnvBuilder {
        if turn_on {
            self.transform_state.c_oflag |= CS8;
        } else {
            self.transform_state.c_oflag &= !CS8;
        }
        self
    }

    pub fn enable_raw_mode(self) -> EnvBuilder {
        self
            .echo(false)
            .i_canon(false)
            .i_sig(false)
            .i_exten(false)
            .i_xon(false)
            .i_crnl(false)
            .o_post(false) // assuming println! does some magic here...
            .brkint(false)
            .inpck(false)
            .i_strip(false)
            .cs8(true)
    }
    
    pub fn set_env(self) ->  Result<EnvBuilder, io::Error> {
        tcsetattr(self.fd, TCSAFLUSH, &self.transform_state)?;
        Ok(self)
    }

    pub fn restore(self) ->  Result<(), io::Error> {
        tcsetattr(self.fd, TCSAFLUSH, &self.original_state)?;
        Ok(())
    }
}