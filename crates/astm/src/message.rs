use encoding::all::{ASCII, WINDOWS_1251};
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use std::str::FromStr;

use crate::{ctrl, ASTMError, CharEncoding, CtrlChar, Result};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frame {
    // Frame number.
    pub(crate) number: u8,
    // Data content of message.
    pub(crate) data: String,
    // Is last frame in message.
    pub(crate) last: bool,
}

impl std::fmt::Display for Frame {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.data)
    }
}

impl Frame {
    pub(crate) fn deserialize(src: &[u8], encoding: CharEncoding) -> Result<Self> {
        let mut checksum = 0usize;
        let mut frame = Frame::default();
        let mut chars = src.iter();

        // <STX>
        match chars.next() {
            Some(t) if *t == ctrl!(STX) => Ok(()),
            Some(_) => Err(ASTMError::InvalidSTXCharacter),
            None => Err(ASTMError::MissingSTXCharacter),
        }?;

        // Frame Number
        frame.number = match chars.next() {
            Some(t) => {
                checksum += *t as usize;
                match char::from_u32(*t as u32) {
                    Some(char) => match char.to_digit(10) {
                        Some(t) if t < 8 => Ok(t as u8),
                        Some(_) => Err(ASTMError::InvalidFrameNumber),
                        None => Err(ASTMError::InvalidFrameNumberNotNumeric),
                    },
                    None => Err(ASTMError::InvalidFrameNumber),
                }
            }
            None => Err(ASTMError::MissingFrameNumber),
        }?;

        // Data Content
        let mut content: Vec<u8> = Vec::new();

        for t in &mut chars {
            checksum += *t as usize;
            if *t == ctrl!(ETB) {
                break;
            } else if *t == ctrl!(ETX) {
                frame.last = true;
                break;
            } else {
                content.push(*t);
            }
        }

        if content.len() > 63993 {
            return Err(ASTMError::OversizedMessage);
        }

        frame.data = match encoding {
            CharEncoding::ASCII => ASCII
                .decode(&content, DecoderTrap::Strict)
                .map_err(ASTMError::DecodingASCIIFrame)?,
            CharEncoding::Windows1251 => WINDOWS_1251
                .decode(&content, DecoderTrap::Strict)
                .map_err(ASTMError::DecodingWINDOWS1251Frame)?,
            CharEncoding::UTF8 => std::str::from_utf8(&content)
                .map(String::from)
                .map_err(ASTMError::DecodingUTF8Frame)?,
        };

        // C1 Checksum
        let c1 = match chars.next() {
            Some(t) => match char::from_u32(*t as u32) {
                Some(char) => match char.to_digit(16) {
                    Some(c1) => Ok(c1 as u8),
                    None => Err(ASTMError::InvalidC1ChecksumValue),
                },
                None => Err(ASTMError::MissingC1ChecksumValue),
            },
            None => Err(ASTMError::MissingC1ChecksumValue),
        }?;

        // C2 Checksum
        let c2 = match chars.next() {
            Some(t) => match char::from_u32(*t as u32) {
                Some(char) => match char.to_digit(16) {
                    Some(c2) => Ok(c2 as u8),
                    None => Err(ASTMError::InvalidC2ChecksumValue),
                },
                None => Err(ASTMError::MissingC2ChecksumValue),
            },
            None => Err(ASTMError::MissingC2ChecksumValue),
        }?;

        // <CR>
        match chars.next() {
            Some(t) if *t == ctrl!(CR) as u8 => Ok(()),
            Some(_) => Err(ASTMError::InvalidCRCharacter),
            None => Err(ASTMError::MissingCRCharacter),
        }?;

        // <LF>
        match chars.next() {
            Some(t) if *t == ctrl!(LF) as u8 => Ok(()),
            Some(_) => Err(ASTMError::InvalidLFCharacter),
            None => Err(ASTMError::MissingLFCharacter),
        }?;

        // Validate frame
        let modulo = (checksum % 256) as u8;
        let text = format!("{:02X}", modulo);

        let mut chars = text.chars();
        let calc_c1 = chars.next().unwrap().to_digit(16).unwrap() as u8;
        let calc_c2 = chars.next().unwrap().to_digit(16).unwrap() as u8;

        if c1 == calc_c1 && c2 == calc_c2 {
            Ok(frame)
        } else {
            Err(ASTMError::DefectiveFrame(text))
        }
    }

    pub(crate) fn serialize(&self, encoding: CharEncoding) -> Result<Vec<u8>> {
        let mut checksum = 0usize;

        // <STX>
        let mut dst = vec![ctrl!(STX)];

        // Frame Number
        let char = self.number + 0x30;
        dst.push(char);
        checksum += char as usize;

        // Data Content
        let mut encoded = match encoding {
            CharEncoding::ASCII => ASCII
                .encode(&self.data, EncoderTrap::Strict)
                .map_err(ASTMError::EncodingASCIIFrame)?,
            CharEncoding::Windows1251 => WINDOWS_1251
                .encode(&self.data, EncoderTrap::Strict)
                .map_err(ASTMError::EncodingWINDOWS1251Frame)?,
            CharEncoding::UTF8 => self.data.as_bytes().to_vec(),
        };

        for char in &encoded {
            checksum += *char as usize;
        }

        dst.append(&mut encoded);

        // <ETB> or <ETX>
        let data_end = if self.last { ctrl!(ETX) } else { ctrl!(ETB) };

        checksum += data_end as usize;
        dst.push(data_end);

        // Checksum
        let modulo = (checksum % 256) as u8;
        let text = format!("{:02X}", modulo);
        let mut chars = text.chars();
        let c1 = chars.next().unwrap() as u8;
        let c2 = chars.next().unwrap() as u8;

        dst.push(c1);
        dst.push(c2);

        // <CR>
        dst.push(ctrl!(CR));

        // <LF>
        dst.push(ctrl!(LF));

        Ok(dst)
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn is_last(&self) -> bool {
        self.last
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct Message {
    pub(crate) frames: Vec<Frame>,
}

impl std::fmt::Display for Message {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for frame in &self.frames {
            let value = frame.data();
            fmt.write_str(&value)?
        }

        Ok(())
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for frame in &self.frames {
            let value = frame.data().replace("\r", "<CR>\n");
            fmt.write_str(&value)?
        }

        Ok(())
    }
}

impl FromStr for Message {
    type Err = ASTMError;

    fn from_str(src: &str) -> std::result::Result<Self, Self::Err> {
        let mut counter = 1;
        let mut dst = Self::default();

        for full_line in src.trim().split('\r') {
            let lines: Vec<&str> = if full_line.len() > 63900 {
                full_line
                    .as_bytes()
                    .chunks(63900)
                    .filter_map(|t| std::str::from_utf8(t).ok())
                    .collect()
            } else {
                vec![full_line]
            };

            let size = lines.len();

            for (index, line) in lines.into_iter().enumerate() {
                dst.push_frame(Frame {
                    number: counter,
                    data: format!("{}\r", line),
                    last: (index + 1 == size),
                });

                // 1, 2, .. , 7, 0, 1
                counter += 1;
                if counter == 8 {
                    counter = 0;
                }
            }
        }

        Ok(dst)
    }
}

impl Message {
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub(crate) fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub(crate) fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}

