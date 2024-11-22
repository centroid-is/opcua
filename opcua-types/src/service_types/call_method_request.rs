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
pub struct CallMethodRequest {
    pub object_id: opcua::types::node_id::NodeId,
    pub method_id: opcua::types::node_id::NodeId,
    pub input_arguments: Option<Vec<opcua::types::variant::Variant>>,
}
impl opcua::types::MessageInfo for CallMethodRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CallMethodRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CallMethodRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CallMethodRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for CallMethodRequest {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.object_id.byte_len(ctx);
        size += self.method_id.byte_len(ctx);
        size += self.input_arguments.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.object_id.encode(stream, ctx)?;
        size += self.method_id.encode(stream, ctx)?;
        size += self.input_arguments.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for CallMethodRequest {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            object_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            method_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            input_arguments: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
