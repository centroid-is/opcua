// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct XVType {
    pub x: f64,
    pub value: f32,
}
impl crate::types::MessageInfo for XVType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::XVType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for XVType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.x.byte_len();
        size += self.value.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.x.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let x = <f64 as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let value = <f32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self { x, value })
    }
}
