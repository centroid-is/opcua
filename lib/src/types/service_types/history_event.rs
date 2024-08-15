// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct HistoryEvent {
    pub events: Option<Vec<super::history_event_field_list::HistoryEventFieldList>>,
}
impl crate::types::MessageInfo for HistoryEvent {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::HistoryEvent_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for HistoryEvent {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.events.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.events.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let events = <Option<
            Vec<super::history_event_field_list::HistoryEventFieldList>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { events })
    }
}
