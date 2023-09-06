pub struct KeyPress(KeyCode); 

impl KeyPress {
    pub fn new(c: u8) -> KeyPress {
        let c = c as char; 

        if !c.is_control() {
            return KeyPress(KeyCode::from_char(c));
        }

        match c {
            '\x00' => KeyPress(KeyCode::from_control(KeyType::NullCharacter, c)), 
            '\x01' => KeyPress(KeyCode::from_control(KeyType::StartOfHeader, c)),
            '\x02' => KeyPress(KeyCode::from_control(KeyType::StartOfText, c)),
            '\x03' => KeyPress(KeyCode::from_control(KeyType::EndOfText, c)),
            '\x04' => KeyPress(KeyCode::from_control(KeyType::EndOfTransmission, c)),
            '\x05' => KeyPress(KeyCode::from_control(KeyType::Enquiry, c)),
            '\x06' => KeyPress(KeyCode::from_control(KeyType::Acknowledge, c)),
            '\x07' => KeyPress(KeyCode::from_control(KeyType::Bell, c)),
            '\x08' => KeyPress(KeyCode::from_control(KeyType::Backspace, c)),
            '\x09' => KeyPress(KeyCode::from_control(KeyType::HorizontalTab, c)),
            '\x0a' => KeyPress(KeyCode::from_control(KeyType::LineFeed, c)),
            '\x0b' => KeyPress(KeyCode::from_control(KeyType::VerticalTab, c)),
            '\x0c' => KeyPress(KeyCode::from_control(KeyType::FormFeed, c)),
            '\x0d' => KeyPress(KeyCode::from_control(KeyType::CarriageReturn, c)),
            '\x0e' => KeyPress(KeyCode::from_control(KeyType::ShiftOut, c)),
            '\x0f' => KeyPress(KeyCode::from_control(KeyType::ShiftIn, c)),
            '\x10' => KeyPress(KeyCode::from_control(KeyType::DataLinkEscape, c)),
            '\x11' => KeyPress(KeyCode::from_control(KeyType::TransmitOn, c)),
            '\x12' => KeyPress(KeyCode::from_control(KeyType::DeviceControl2, c)),
            '\x13' => KeyPress(KeyCode::from_control(KeyType::TransmitOff, c)),
            '\x14' => KeyPress(KeyCode::from_control(KeyType::DeviceControl4, c)),
            '\x15' => KeyPress(KeyCode::from_control(KeyType::NegativeAcknowledge, c)),
            '\x16' => KeyPress(KeyCode::from_control(KeyType::SynchronousIdle, c)),
            '\x17' => KeyPress(KeyCode::from_control(KeyType::EndOfTransmissionBlock, c)),
            '\x18' => KeyPress(KeyCode::from_control(KeyType::Cancel, c)),
            '\x19' => KeyPress(KeyCode::from_control(KeyType::EndOfMedium, c)),
            '\x1a' => KeyPress(KeyCode::from_control(KeyType::Substitute, c)),
            '\x1b' => KeyPress(KeyCode::from_control(KeyType::Escape, c)),
            '\x1c' => KeyPress(KeyCode::from_control(KeyType::FileSeparator, c)),
            '\x1d' => KeyPress(KeyCode::from_control(KeyType::GroupSeparator, c)),
            '\x1e' => KeyPress(KeyCode::from_control(KeyType::RecordSeparator, c)),
            '\x1f' => KeyPress(KeyCode::from_control(KeyType::UnitSeparator, c)),
            '\x7f' => KeyPress(KeyCode::from_control(KeyType::Delete, c)),
            _ => KeyPress(KeyCode::from_control(KeyType::NoOp, c)) // catch all...
        }
    }

    pub fn get_byte(&self) -> u8 {
        self.0.get_byte()
    }

    pub fn get_char(&self) -> char {
        self.0.get_char()
    }

    pub fn get_type(&self) -> KeyType {
        self.0.key_type
    }

    pub fn is_type(&self, t: KeyType) -> bool {
        self.0.key_type == t
    }
}


pub struct KeyCode {
    pub key_type: KeyType,
    inner: char,
}

impl KeyCode {
    pub fn from_control(key_type: KeyType, inner: char) -> KeyCode {
        KeyCode {
            key_type,
            inner
        }
    }

    pub fn from_char(inner: char) -> KeyCode {
        KeyCode {
            key_type: KeyType::Char,
            inner
        }
    }

    pub fn get_byte(&self) -> u8 {
        self.inner as u8
    }

    pub fn get_char(&self) -> char {
        self.inner as char
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyType {
    NullCharacter,
    StartOfHeader,
    StartOfText,
    EndOfText,
    EndOfTransmission,
    Enquiry,
    Acknowledge,
    Bell,
    Backspace,
    HorizontalTab,
    LineFeed,
    VerticalTab,
    FormFeed,
    CarriageReturn,
    ShiftOut,
    ShiftIn,
    DataLinkEscape,
    TransmitOn,
    DeviceControl2,
    TransmitOff,
    DeviceControl4,
    NegativeAcknowledge,
    SynchronousIdle,
    EndOfTransmissionBlock,
    Cancel,
    EndOfMedium,
    Substitute,
    Escape,
    FileSeparator,
    GroupSeparator,
    RecordSeparator,
    UnitSeparator,
    Space,
    // some terminals will send 0x7f for backspace, others will send 0x08, others may send an escape sequence...
    Delete,
    Char,  // For any other character
    NoOp, // any character that can't be caputured here...
}



