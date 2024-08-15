// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct PublishedVariableDataType {
    pub published_variable: crate::types::node_id::NodeId,
    pub attribute_id: u32,
    pub sampling_interval_hint: f64,
    pub deadband_type: u32,
    pub deadband_value: f64,
    pub index_range: crate::types::string::UAString,
    pub substitute_value: crate::types::variant::Variant,
    pub meta_data_properties: Option<Vec<crate::types::qualified_name::QualifiedName>>,
}
impl crate::types::MessageInfo for PublishedVariableDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PublishedVariableDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for PublishedVariableDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.published_variable.byte_len();
        size += self.attribute_id.byte_len();
        size += self.sampling_interval_hint.byte_len();
        size += self.deadband_type.byte_len();
        size += self.deadband_value.byte_len();
        size += self.index_range.byte_len();
        size += self.substitute_value.byte_len();
        size += self.meta_data_properties.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.published_variable.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.sampling_interval_hint.encode(stream)?;
        size += self.deadband_type.encode(stream)?;
        size += self.deadband_value.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.substitute_value.encode(stream)?;
        size += self.meta_data_properties.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let published_variable = <crate::types::node_id::NodeId as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let attribute_id = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let sampling_interval_hint = <f64 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let deadband_type = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let deadband_value = <f64 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let index_range = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let substitute_value = <crate::types::variant::Variant as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let meta_data_properties = <Option<
            Vec<crate::types::qualified_name::QualifiedName>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            published_variable,
            attribute_id,
            sampling_interval_hint,
            deadband_type,
            deadband_value,
            index_range,
            substitute_value,
            meta_data_properties,
        })
    }
}
