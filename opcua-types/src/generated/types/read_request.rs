// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ReadRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub max_age: f64,
    pub timestamps_to_return: super::enums::TimestampsToReturn,
    pub nodes_to_read: Option<Vec<super::read_value_id::ReadValueId>>,
}
impl opcua::types::MessageInfo for ReadRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadRequest_Encoding_DefaultXml
    }
}
