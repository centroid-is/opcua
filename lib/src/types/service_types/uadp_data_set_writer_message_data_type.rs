// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct UadpDataSetWriterMessageDataType {
    pub data_set_message_content_mask: super::enums::UadpDataSetMessageContentMask,
    pub configured_size: u16,
    pub network_message_number: u16,
    pub data_set_offset: u16,
}
impl crate::types::BinaryEncoder for UadpDataSetWriterMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.byte_len();
        size += self.configured_size.byte_len();
        size += self.network_message_number.byte_len();
        size += self.data_set_offset.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.encode(stream)?;
        size += self.configured_size.encode(stream)?;
        size += self.network_message_number.encode(stream)?;
        size += self.data_set_offset.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let data_set_message_content_mask = <super::enums::UadpDataSetMessageContentMask as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let configured_size = <u16 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let network_message_number = <u16 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_offset = <u16 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            data_set_message_content_mask,
            configured_size,
            network_message_number,
            data_set_offset,
        })
    }
}
