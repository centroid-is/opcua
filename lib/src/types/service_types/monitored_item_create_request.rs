// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemCreateRequest {
    pub item_to_monitor: super::read_value_id::ReadValueId,
    pub monitoring_mode: super::enums::MonitoringMode,
    pub requested_parameters: super::monitoring_parameters::MonitoringParameters,
}
impl crate::types::MessageInfo for MonitoredItemCreateRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::MonitoredItemCreateRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for MonitoredItemCreateRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.item_to_monitor.byte_len();
        size += self.monitoring_mode.byte_len();
        size += self.requested_parameters.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.item_to_monitor.encode(stream)?;
        size += self.monitoring_mode.encode(stream)?;
        size += self.requested_parameters.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let item_to_monitor = <super::read_value_id::ReadValueId as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let monitoring_mode = <super::enums::MonitoringMode as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let requested_parameters = <super::monitoring_parameters::MonitoringParameters as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            item_to_monitor,
            monitoring_mode,
            requested_parameters,
        })
    }
}
