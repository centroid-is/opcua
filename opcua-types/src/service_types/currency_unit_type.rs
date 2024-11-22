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
pub struct CurrencyUnitType {
    pub numeric_code: i16,
    pub exponent: i8,
    pub alphabetic_code: opcua::types::string::UAString,
    pub currency: opcua::types::localized_text::LocalizedText,
}
impl opcua::types::MessageInfo for CurrencyUnitType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for CurrencyUnitType {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.numeric_code.byte_len(ctx);
        size += self.exponent.byte_len(ctx);
        size += self.alphabetic_code.byte_len(ctx);
        size += self.currency.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.numeric_code.encode(stream, ctx)?;
        size += self.exponent.encode(stream, ctx)?;
        size += self.alphabetic_code.encode(stream, ctx)?;
        size += self.currency.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for CurrencyUnitType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            numeric_code: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            exponent: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            alphabetic_code: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            currency: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
