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
pub struct IssuedIdentityToken {
    pub policy_id: opcua::types::string::UAString,
    pub token_data: opcua::types::byte_string::ByteString,
    pub encryption_algorithm: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for IssuedIdentityToken {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for IssuedIdentityToken {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.policy_id.byte_len(ctx);
        size += self.token_data.byte_len(ctx);
        size += self.encryption_algorithm.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.policy_id.encode(stream, ctx)?;
        size += self.token_data.encode(stream, ctx)?;
        size += self.encryption_algorithm.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for IssuedIdentityToken {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            policy_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            token_data: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            encryption_algorithm: opcua::types::BinaryDecodable::decode(stream, ctx)?,
        })
    }
}
