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
pub struct PublishedEventsDataType {
    pub event_notifier: opcua::types::node_id::NodeId,
    pub selected_fields: Option<
        Vec<super::simple_attribute_operand::SimpleAttributeOperand>,
    >,
    pub filter: super::content_filter::ContentFilter,
}
impl opcua::types::MessageInfo for PublishedEventsDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedEventsDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedEventsDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedEventsDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for PublishedEventsDataType {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.event_notifier.byte_len(ctx);
        size += self.selected_fields.byte_len(ctx);
        size += self.filter.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.event_notifier.encode(stream, ctx)?;
        size += self.selected_fields.encode(stream, ctx)?;
        size += self.filter.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for PublishedEventsDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            event_notifier: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            selected_fields: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            filter: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
