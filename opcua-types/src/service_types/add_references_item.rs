// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
pub struct AddReferencesItem {
    pub source_node_id: opcua::types::node_id::NodeId,
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub is_forward: bool,
    pub target_server_uri: opcua::types::string::UAString,
    pub target_node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub target_node_class: super::enums::NodeClass,
}
impl opcua::types::MessageInfo for AddReferencesItem {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AddReferencesItem_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for AddReferencesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.source_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.target_server_uri.byte_len();
        size += self.target_node_id.byte_len();
        size += self.target_node_class.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.source_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.target_server_uri.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.target_node_class.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let source_node_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let reference_type_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let is_forward = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let target_server_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let target_node_id = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let target_node_class = <super::enums::NodeClass as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            source_node_id,
            reference_type_id,
            is_forward,
            target_server_uri,
            target_node_id,
            target_node_class,
        })
    }
}
