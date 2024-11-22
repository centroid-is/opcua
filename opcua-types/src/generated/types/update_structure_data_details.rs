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
pub struct UpdateStructureDataDetails {
    pub node_id: opcua::types::node_id::NodeId,
    pub perform_insert_replace: super::enums::PerformUpdateType,
    pub update_values: Option<Vec<opcua::types::data_value::DataValue>>,
}
impl opcua::types::MessageInfo for UpdateStructureDataDetails {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UpdateStructureDataDetails_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UpdateStructureDataDetails_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UpdateStructureDataDetails_Encoding_DefaultXml
    }
}
