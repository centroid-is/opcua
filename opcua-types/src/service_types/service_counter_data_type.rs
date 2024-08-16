// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct ServiceCounterDataType {
    pub total_count: u32,
    pub error_count: u32,
}
impl opcua::types::MessageInfo for ServiceCounterDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ServiceCounterDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ServiceCounterDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.total_count.byte_len();
        size += self.error_count.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.total_count.encode(stream)?;
        size += self.error_count.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let total_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let error_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self { total_count, error_count })
    }
}
