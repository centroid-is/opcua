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
pub struct FindServersOnNetworkResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub last_counter_reset_time: opcua::types::date_time::DateTime,
    pub servers: Option<Vec<super::server_on_network::ServerOnNetwork>>,
}
impl opcua::types::MessageInfo for FindServersOnNetworkResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkResponse_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for FindServersOnNetworkResponse {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len(ctx);
        size += self.last_counter_reset_time.byte_len(ctx);
        size += self.servers.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream, ctx)?;
        size += self.last_counter_reset_time.encode(stream, ctx)?;
        size += self.servers.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for FindServersOnNetworkResponse {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header: opcua::types::response_header::ResponseHeader = opcua::types::BinaryDecodable::decode(
            stream,
            ctx,
        )?;
        let __request_handle = response_header.request_handle;
        Ok(Self {
            response_header,
            last_counter_reset_time: opcua::types::BinaryDecodable::decode(stream, ctx)
                .map_err(|e| e.with_request_handle(__request_handle))?,
            servers: opcua::types::BinaryDecodable::decode(stream, ctx)
                .map_err(|e| e.with_request_handle(__request_handle))?,
        })
    }
}
