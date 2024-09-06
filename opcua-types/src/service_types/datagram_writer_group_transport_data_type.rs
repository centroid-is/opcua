// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct DatagramWriterGroupTransportDataType {
    pub message_repeat_count: u8,
    pub message_repeat_delay: f64,
}
impl opcua::types::MessageInfo for DatagramWriterGroupTransportDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DatagramWriterGroupTransportDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for DatagramWriterGroupTransportDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.message_repeat_count.byte_len();
        size += self.message_repeat_delay.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.message_repeat_count.encode(stream)?;
        size += self.message_repeat_delay.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let message_repeat_count = <u8 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let message_repeat_delay = <f64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            message_repeat_count,
            message_repeat_delay,
        })
    }
}
