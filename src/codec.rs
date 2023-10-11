use std::{io, str};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Parse")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Utf8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Io")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("io")]
    Io(#[from] io::Error),
}

pub struct EnumCodec;

impl Decoder for EnumCodec {
    type Item = usize;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let mut line = src.split_to(n + 1);
            // println!("line: {:?}", line);
            let mut line = line.split_to(line.len() - 1);
            if line.is_empty() {
                return Ok(None);
            }
            let line = if line[line.len() - 1] == b'\r' {
                line.split_to(line.len() - 1)
            } else {
                line
            };
            let str = str::from_utf8(line.as_ref())?;

            return Ok(Some(str.parse::<usize>()?));
        }
        Ok(None)
    }
}

impl Encoder<String> for EnumCodec {
    type Error = EncodeError;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.as_bytes());
        Ok(())
    }
}

impl Encoder<usize> for EnumCodec {
    type Error = EncodeError;
    fn encode(&mut self, item: usize, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.to_string().as_bytes());
        dst.extend_from_slice(b"\r\n");
        Ok(())
    }
}
