// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct UserNameIdentityToken {
    pub policy_id: opcua::types::string::UAString,
    pub user_name: opcua::types::string::UAString,
    pub password: opcua::types::byte_string::ByteString,
    pub encryption_algorithm: opcua::types::string::UAString,
}
impl opcua::types::BinaryEncoder for UserNameIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.policy_id.byte_len();
        size += self.user_name.byte_len();
        size += self.password.byte_len();
        size += self.encryption_algorithm.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.policy_id.encode(stream)?;
        size += self.user_name.encode(stream)?;
        size += self.password.encode(stream)?;
        size += self.encryption_algorithm.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let policy_id = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let user_name = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let password = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let encryption_algorithm = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            policy_id,
            user_name,
            password,
            encryption_algorithm,
        })
    }
}
