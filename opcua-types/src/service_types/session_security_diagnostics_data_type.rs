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
pub struct SessionSecurityDiagnosticsDataType {
    pub session_id: opcua::types::node_id::NodeId,
    pub client_user_id_of_session: opcua::types::string::UAString,
    pub client_user_id_history: Option<Vec<opcua::types::string::UAString>>,
    pub authentication_mechanism: opcua::types::string::UAString,
    pub encoding: opcua::types::string::UAString,
    pub transport_protocol: opcua::types::string::UAString,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_policy_uri: opcua::types::string::UAString,
    pub client_certificate: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for SessionSecurityDiagnosticsDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionSecurityDiagnosticsDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for SessionSecurityDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.session_id.byte_len();
        size += self.client_user_id_of_session.byte_len();
        size += self.client_user_id_history.byte_len();
        size += self.authentication_mechanism.byte_len();
        size += self.encoding.byte_len();
        size += self.transport_protocol.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_policy_uri.byte_len();
        size += self.client_certificate.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.session_id.encode(stream)?;
        size += self.client_user_id_of_session.encode(stream)?;
        size += self.client_user_id_history.encode(stream)?;
        size += self.authentication_mechanism.encode(stream)?;
        size += self.encoding.encode(stream)?;
        size += self.transport_protocol.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_policy_uri.encode(stream)?;
        size += self.client_certificate.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let session_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let client_user_id_of_session = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let client_user_id_history = <Option<
            Vec<opcua::types::string::UAString>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let authentication_mechanism = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let encoding = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let transport_protocol = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_mode = <super::enums::MessageSecurityMode as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_policy_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let client_certificate = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            session_id,
            client_user_id_of_session,
            client_user_id_history,
            authentication_mechanism,
            encoding,
            transport_protocol,
            security_mode,
            security_policy_uri,
            client_certificate,
        })
    }
}
