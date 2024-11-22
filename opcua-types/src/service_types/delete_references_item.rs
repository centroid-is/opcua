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
pub struct DeleteReferencesItem {
    pub source_node_id: opcua::types::node_id::NodeId,
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub is_forward: bool,
    pub target_node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub delete_bidirectional: bool,
}
impl opcua::types::MessageInfo for DeleteReferencesItem {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for DeleteReferencesItem {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.source_node_id.byte_len(ctx);
        size += self.reference_type_id.byte_len(ctx);
        size += self.is_forward.byte_len(ctx);
        size += self.target_node_id.byte_len(ctx);
        size += self.delete_bidirectional.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.source_node_id.encode(stream, ctx)?;
        size += self.reference_type_id.encode(stream, ctx)?;
        size += self.is_forward.encode(stream, ctx)?;
        size += self.target_node_id.encode(stream, ctx)?;
        size += self.delete_bidirectional.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for DeleteReferencesItem {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            source_node_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            reference_type_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            is_forward: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            target_node_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            delete_bidirectional: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
