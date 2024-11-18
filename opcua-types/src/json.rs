use std::{
    io::{Read, Write},
    num::{ParseFloatError, ParseIntError},
};

pub use crate::Context;
use log::warn;
use struson::writer::JsonNumberError;
pub use struson::{
    reader::{JsonReader, JsonStreamReader, ValueType},
    writer::{JsonStreamWriter, JsonWriter},
};

use crate::{EncodingError, EncodingResult, StatusCode};

pub trait JsonEncodable {
    #[allow(unused)]
    fn encode(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        ctx: &crate::Context<'_>,
    ) -> EncodingResult<()>;

    fn is_null_json(&self) -> bool {
        false
    }
}

impl From<struson::reader::ReaderError> for EncodingError {
    fn from(value: struson::reader::ReaderError) -> Self {
        warn!("Json decoding error {}", value);
        Self::from(StatusCode::BadDecodingError)
    }
}

impl From<std::io::Error> for EncodingError {
    fn from(value: std::io::Error) -> Self {
        warn!("IO error during encoding/decoding: {value}");
        Self::from(StatusCode::BadEncodingError)
    }
}

impl From<ParseIntError> for EncodingError {
    fn from(value: ParseIntError) -> Self {
        warn!("Number out of range: {value}");
        Self::from(StatusCode::BadDecodingError)
    }
}

impl From<ParseFloatError> for EncodingError {
    fn from(value: ParseFloatError) -> Self {
        warn!("Invalid floating point number: {value}");
        Self::from(StatusCode::BadDecodingError)
    }
}

impl From<JsonNumberError> for EncodingError {
    fn from(value: JsonNumberError) -> Self {
        warn!("Invalid JSON number: {value}");
        Self::from(StatusCode::BadEncodingError)
    }
}

pub trait JsonDecodable: Sized {
    #[allow(unused)]
    fn decode(
        stream: &mut JsonStreamReader<&mut dyn Read>,
        ctx: &Context<'_>,
    ) -> EncodingResult<Self>;
}

impl<T> JsonEncodable for Option<T>
where
    T: JsonEncodable,
{
    fn encode(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        ctx: &crate::Context<'_>,
    ) -> EncodingResult<()> {
        match self {
            Some(s) => s.encode(stream, ctx),
            None => Ok(stream.null_value()?),
        }
    }

    fn is_null_json(&self) -> bool {
        self.is_none()
    }
}

impl<T> JsonDecodable for Option<T>
where
    T: JsonDecodable,
{
    fn decode(
        stream: &mut JsonStreamReader<&mut dyn Read>,
        ctx: &Context<'_>,
    ) -> EncodingResult<Self> {
        match stream.peek()? {
            ValueType::Null => {
                stream.next_null()?;
                Ok(None)
            }
            _ => Ok(Some(T::decode(stream, ctx)?)),
        }
    }
}

impl<T> JsonEncodable for Vec<T>
where
    T: JsonEncodable,
{
    fn encode(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        ctx: &crate::Context<'_>,
    ) -> EncodingResult<()> {
        stream.begin_array()?;
        for elem in self {
            elem.encode(stream, ctx)?;
        }
        stream.end_array()?;
        Ok(())
    }
}

impl<T> JsonDecodable for Vec<T>
where
    T: JsonDecodable,
{
    fn decode(
        stream: &mut JsonStreamReader<&mut dyn Read>,
        ctx: &Context<'_>,
    ) -> EncodingResult<Self> {
        if stream.peek()? == ValueType::Null {
            stream.next_null()?;
            return Ok(Vec::new());
        }

        let mut res = Vec::new();
        stream.begin_array()?;
        while stream.has_next()? {
            res.push(T::decode(stream, ctx)?);
        }
        stream.end_array()?;

        Ok(res)
    }
}

macro_rules! json_enc_float {
    ($t:ty) => {
        impl JsonEncodable for $t {
            fn encode(
                &self,
                stream: &mut JsonStreamWriter<&mut dyn Write>,
                _ctx: &crate::Context<'_>,
            ) -> EncodingResult<()> {
                if self.is_infinite() {
                    if self.is_sign_positive() {
                        stream.string_value("Infinity")?;
                    } else {
                        stream.string_value("-Infinity")?;
                    }
                } else if self.is_nan() {
                    stream.string_value("NaN")?;
                } else {
                    stream.fp_number_value(*self)?;
                }

                Ok(())
            }
        }

        impl JsonDecodable for $t {
            fn decode(
                stream: &mut JsonStreamReader<&mut dyn Read>,
                _ctx: &Context<'_>,
            ) -> EncodingResult<Self> {
                if stream.peek()? == ValueType::String {
                    let v = stream.next_str()?;
                    match v {
                        "Infinity" => Ok(Self::INFINITY),
                        "-Infinity" => Ok(Self::NEG_INFINITY),
                        "NaN" => Ok(Self::NAN),
                        // Not technically spec, but to optimize interoperability, try to
                        // parse the number as a float
                        r => Ok(r.parse()?),
                    }
                } else {
                    Ok(stream.next_number()??)
                }
            }
        }
    };
}

macro_rules! json_enc_number {
    ($t:ty) => {
        impl JsonEncodable for $t {
            fn encode(
                &self,
                stream: &mut JsonStreamWriter<&mut dyn Write>,
                _ctx: &crate::Context<'_>,
            ) -> EncodingResult<()> {
                stream.number_value(*self)?;
                Ok(())
            }
        }

        impl JsonDecodable for $t {
            fn decode(
                stream: &mut JsonStreamReader<&mut dyn Read>,
                _ctx: &Context<'_>,
            ) -> EncodingResult<Self> {
                Ok(stream.next_number()??)
            }
        }
    };
}

json_enc_number!(u8);
json_enc_number!(u16);
json_enc_number!(u32);
json_enc_number!(u64);
json_enc_number!(i8);
json_enc_number!(i16);
json_enc_number!(i32);
json_enc_number!(i64);
json_enc_float!(f32);
json_enc_float!(f64);

impl JsonEncodable for String {
    fn encode(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        _ctx: &crate::Context<'_>,
    ) -> EncodingResult<()> {
        stream.string_value(self.as_str())?;
        Ok(())
    }
}

impl JsonDecodable for String {
    fn decode(
        stream: &mut JsonStreamReader<&mut dyn Read>,
        _ctx: &Context<'_>,
    ) -> EncodingResult<Self> {
        Ok(stream.next_string()?)
    }
}
