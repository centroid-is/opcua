// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct AdditionalParametersType {
    pub parameters: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl opcua::types::MessageInfo for AdditionalParametersType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AdditionalParametersType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for AdditionalParametersType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.parameters.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.parameters.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let parameters = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { parameters })
    }
}
