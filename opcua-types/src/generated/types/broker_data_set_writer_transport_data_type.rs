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
pub struct BrokerDataSetWriterTransportDataType {
    pub queue_name: opcua::types::string::UAString,
    pub resource_uri: opcua::types::string::UAString,
    pub authentication_profile_uri: opcua::types::string::UAString,
    pub requested_delivery_guarantee: super::enums::BrokerTransportQualityOfService,
    pub meta_data_queue_name: opcua::types::string::UAString,
    pub meta_data_update_time: f64,
}
impl opcua::types::MessageInfo for BrokerDataSetWriterTransportDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultXml
    }
}
