pub enum InputType {
    Ascii(AsciiKey),
    Ansi(EscapeSequence),
}
/*
Represents a single key press, with the exepction of those keys that send 
escape sequences.  This struct is used to represent the key press in the
buffer.  The escape sequences are handled by the EscapeSequence struct.
*/
#[derive(Clone, Copy, PartialEq)]
pub enum AsciiKey {
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
    // some terminals will send 0x7f for backspace others will send 0x08 others may send an escape sequence...
    Delete,
    Char(char),  // For any other character
    NoOp(char) // any character that can't be caputured here...
}

impl AsciiKey {
    pub fn new(c: &[u8]) -> AsciiKey {
        if c.len() > 1 {
            panic!("Can't convert multi-byte keypress to char");
        }
        
        let c = c[0] as char; 

        if !c.is_control() {
            return AsciiKey::Char(c);
        }

        match c {
            '\x00' => AsciiKey::NullCharacter, 
            '\x01' => AsciiKey::StartOfHeader,
            '\x02' => AsciiKey::StartOfText,
            '\x03' => AsciiKey::EndOfText,
            '\x04' => AsciiKey::EndOfTransmission,
            '\x05' => AsciiKey::Enquiry,
            '\x06' => AsciiKey::Acknowledge,
            '\x07' => AsciiKey::Bell,
            '\x08' => AsciiKey::Backspace,
            '\x09' => AsciiKey::HorizontalTab,
            '\x0a' => AsciiKey::LineFeed,
            '\x0b' => AsciiKey::VerticalTab,
            '\x0c' => AsciiKey::FormFeed,
            '\x0d' => AsciiKey::CarriageReturn,
            '\x0e' => AsciiKey::ShiftOut,
            '\x0f' => AsciiKey::ShiftIn,
            '\x10' => AsciiKey::DataLinkEscape,
            '\x11' => AsciiKey::TransmitOn,
            '\x12' => AsciiKey::DeviceControl2,
            '\x13' => AsciiKey::TransmitOff,
            '\x14' => AsciiKey::DeviceControl4,
            '\x15' => AsciiKey::NegativeAcknowledge,
            '\x16' => AsciiKey::SynchronousIdle,
            '\x17' => AsciiKey::EndOfTransmissionBlock,
            '\x18' => AsciiKey::Cancel,
            '\x19' => AsciiKey::EndOfMedium,
            '\x1a' => AsciiKey::Substitute,
            '\x1b' => AsciiKey::Escape,
            '\x1c' => AsciiKey::FileSeparator,
            '\x1d' => AsciiKey::GroupSeparator,
            '\x1e' => AsciiKey::RecordSeparator,
            '\x1f' => AsciiKey::UnitSeparator,
            '\x7f' => AsciiKey::Delete,
            _ => AsciiKey::NoOp(c) // catch(c).).
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            AsciiKey::NullCharacter => "0x00".to_string(),
            AsciiKey::StartOfHeader => "0x01".to_string(),
            AsciiKey::StartOfText => "0x02".to_string(),
            AsciiKey::EndOfText => "0x03".to_string(),
            AsciiKey::EndOfTransmission => "0x04".to_string(),
            AsciiKey::Enquiry => "0x05".to_string(),
            AsciiKey::Acknowledge => "0x06".to_string(),
            AsciiKey::Bell => "0x07".to_string(),
            AsciiKey::Backspace => "0x08".to_string(),
            AsciiKey::HorizontalTab => "0x09".to_string(),
            AsciiKey::LineFeed => "0x0a".to_string(),
            AsciiKey::VerticalTab => "0x0b".to_string(),
            AsciiKey::FormFeed => "0x0c".to_string(),
            AsciiKey::CarriageReturn => "0x0d".to_string(),
            AsciiKey::ShiftOut => "0x0e".to_string(),
            AsciiKey::ShiftIn => "0x0f".to_string(),
            AsciiKey::DataLinkEscape => "0x10".to_string(),
            AsciiKey::TransmitOn => "0x11".to_string(),
            AsciiKey::DeviceControl2 => "0x12".to_string(),
            AsciiKey::TransmitOff => "0x13".to_string(),
            AsciiKey::DeviceControl4 => "0x14".to_string(),
            AsciiKey::NegativeAcknowledge => "0x15".to_string(),
            AsciiKey::SynchronousIdle => "0x16".to_string(),
            AsciiKey::EndOfTransmissionBlock => "0x17".to_string(),
            AsciiKey::Cancel => "0x18".to_string(),
            AsciiKey::EndOfMedium => "0x19".to_string(),
            AsciiKey::Substitute => "0x1a".to_string(),
            AsciiKey::Escape => "0x1b".to_string(),
            AsciiKey::FileSeparator => "0x1c".to_string(),
            AsciiKey::GroupSeparator => "0x1d".to_string(),
            AsciiKey::RecordSeparator => "0x1e".to_string(),
            AsciiKey::UnitSeparator => "0x1f".to_string(),
            AsciiKey::Space => "0x20".to_string(),
            AsciiKey::Delete => "0x7f".to_string(),
            AsciiKey::Char(c) => c.to_string(),
            AsciiKey::NoOp(c) => c.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if let AsciiKey::NoOp(_) = *self {
            false
        } else {
            true
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum EscapeSequence {
    ArrowUp, // <esc>[#A
    ArrowDown, // <esc>[#B
    ArrowRight, // <esc>[#C
    ArrowLeft, // <esc>[#D
    Delete, // <esc>[3~
    Home, // <esc>[H
    End, // <esc>[F
    PageUp, // <esc>[5~
    PageDown,   // <esc>[6~
    Insert, // <esc>[2~
    F1, // <esc>OP
    F2, // <esc>OQ
    F3, // <esc>OR
    F4, // <esc>OS
    F5, // <esc>[15~
    F6, // <esc>[17~
    F7, // <esc>[18~
    F8, // <esc>[19~
    F9, // <esc>[20~
    F10, // <esc>[21~ 
    F11,    // <esc>[23~
    F12,    // <esc>[24~
    NoOp,   // any other escape sequence
}


impl EscapeSequence {
    pub fn new(c: &[u8]) -> EscapeSequence {
        let as_str = match std::str::from_utf8(c) {
            Ok(s) => s,
            Err(_) => {
                return EscapeSequence::NoOp
            },
        };
        
        match as_str {
            "\x1b[A" => EscapeSequence::ArrowUp,
            "\x1b[B" => EscapeSequence::ArrowDown,
            "\x1b[C" => EscapeSequence::ArrowRight,
            "\x1b[D" => EscapeSequence::ArrowLeft,
            "\x1b[3~" => EscapeSequence::Delete,
            "\x1b[H" => EscapeSequence::Home,
            "\x1b[F" => EscapeSequence::End,
            "\x1b[5~" => EscapeSequence::PageUp,
            "\x1b[6~" => EscapeSequence::PageDown,
            "\x1b[2~" => EscapeSequence::Insert,
            "\x1bOP" => EscapeSequence::F1,
            "\x1bOQ" => EscapeSequence::F2,
            "\x1bOR" => EscapeSequence::F3,
            "\x1bOS" => EscapeSequence::F4,
            "\x1b[15~" => EscapeSequence::F5,
            "\x1b[17~" => EscapeSequence::F6,
            "\x1b[18~" => EscapeSequence::F7,
            "\x1b[19~" => EscapeSequence::F8,
            "\x1b[20~" => EscapeSequence::F9,
            "\x1b[21~" => EscapeSequence::F10,
            "\x1b[23~" => EscapeSequence::F11,
            "\x1b[24~" => EscapeSequence::F12,
            _ => EscapeSequence::NoOp,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            EscapeSequence::ArrowUp => "\x1b[A".to_string(),
            EscapeSequence::ArrowDown => "\x1b[B".to_string(),
            EscapeSequence::ArrowRight => "\x1b[C".to_string(),
            EscapeSequence::ArrowLeft => "\x1b[D".to_string(),
            EscapeSequence::Delete => "\x1b[3~".to_string(),
            EscapeSequence::Home => "\x1b[H".to_string(),
            EscapeSequence::End => "\x1b[F".to_string(),
            EscapeSequence::PageUp => "\x1b[5~".to_string(),
            EscapeSequence::PageDown => "\x1b[6~".to_string(),
            EscapeSequence::Insert => "\x1b[2~".to_string(),
            EscapeSequence::F1 => "\x1bOP".to_string(),
            EscapeSequence::F2 => "\x1bOQ".to_string(),
            EscapeSequence::F3 => "\x1bOR".to_string(),
            EscapeSequence::F4 => "\x1bOS".to_string(),
            EscapeSequence::F5 => "\x1b[15~".to_string(),
            EscapeSequence::F6 => "\x1b[17~".to_string(),
            EscapeSequence::F7 => "\x1b[18~".to_string(),
            EscapeSequence::F8 => "\x1b[19~".to_string(),
            EscapeSequence::F9 => "\x1b[20~".to_string(),
            EscapeSequence::F10 => "\x1b[21~".to_string(),
            EscapeSequence::F11 => "\x1b[23~".to_string(),
            EscapeSequence::F12 => "\x1b[24~".to_string(),
            // Fine to panic for now. Will address this later...
            _ => panic!("Can't convert non-escape sequence to escape sequence"), 
        }
    }

    pub fn is_valid(&self) -> bool {
        if let EscapeSequence::NoOp = *self {
            false
        } else {
            true
        }
    }
}

