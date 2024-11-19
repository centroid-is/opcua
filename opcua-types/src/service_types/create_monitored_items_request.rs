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
pub struct CreateMonitoredItemsRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub subscription_id: u32,
    pub timestamps_to_return: super::enums::TimestampsToReturn,
    pub items_to_create: Option<
        Vec<super::monitored_item_create_request::MonitoredItemCreateRequest>,
    >,
}
impl opcua::types::MessageInfo for CreateMonitoredItemsRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for CreateMonitoredItemsRequest {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len(ctx);
        size += self.subscription_id.byte_len(ctx);
        size += self.timestamps_to_return.byte_len(ctx);
        size += self.items_to_create.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream, ctx)?;
        size += self.subscription_id.encode(stream, ctx)?;
        size += self.timestamps_to_return.encode(stream, ctx)?;
        size += self.items_to_create.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for CreateMonitoredItemsRequest {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header: opcua::types::request_header::RequestHeader = opcua::types::BinaryDecodable::decode(
            stream,
            ctx,
        )?;
        let __request_handle = request_header.request_handle;
        Ok(Self {
            request_header,
            subscription_id: opcua::types::BinaryDecodable::decode(stream, ctx)
                .map_err(|e| e.with_request_handle(__request_handle))?,
            timestamps_to_return: opcua::types::BinaryDecodable::decode(stream, ctx)
                .map_err(|e| e.with_request_handle(__request_handle))?,
            items_to_create: opcua::types::BinaryDecodable::decode(stream, ctx)
                .map_err(|e| e.with_request_handle(__request_handle))?,
        })
    }
}
