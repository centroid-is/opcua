// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[derive(Default)]
pub struct StructureField {
    pub name: opcua::types::string::UAString,
    pub description: opcua::types::localized_text::LocalizedText,
    pub data_type: opcua::types::node_id::NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub max_string_length: u32,
    pub is_optional: bool,
}
impl opcua::types::MessageInfo for StructureField {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::StructureField_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for StructureField {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += self.array_dimensions.byte_len();
        size += self.max_string_length.byte_len();
        size += self.is_optional.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += self.array_dimensions.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.is_optional.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let name = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let description = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_type = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let value_rank = <i32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let array_dimensions = <Option<
            Vec<u32>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let max_string_length = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let is_optional = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            name,
            description,
            data_type,
            value_rank,
            array_dimensions,
            max_string_length,
            is_optional,
        })
    }
}
