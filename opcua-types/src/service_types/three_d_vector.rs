// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ThreeDVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl opcua::types::MessageInfo for ThreeDVector {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDVector_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDVector_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDVector_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ThreeDVector {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.x.byte_len(ctx);
        size += self.y.byte_len(ctx);
        size += self.z.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.x.encode(stream, ctx)?;
        size += self.y.encode(stream, ctx)?;
        size += self.z.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for ThreeDVector {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            x: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            y: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            z: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
