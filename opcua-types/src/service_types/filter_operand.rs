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
pub struct FilterOperand {}
impl opcua::types::MessageInfo for FilterOperand {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FilterOperand_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FilterOperand_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FilterOperand_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for FilterOperand {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        0usize
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        Ok(0)
    }
}
impl opcua::types::BinaryDecodable for FilterOperand {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {})
    }
}
