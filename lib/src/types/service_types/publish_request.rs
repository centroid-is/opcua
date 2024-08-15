// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct PublishRequest {
    pub request_header: crate::types::request_header::RequestHeader,
    pub subscription_acknowledgements: Option<
        Vec<super::subscription_acknowledgement::SubscriptionAcknowledgement>,
    >,
}
impl crate::types::MessageInfo for PublishRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PublishRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for PublishRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.subscription_acknowledgements.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.subscription_acknowledgements.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let request_header = <crate::types::request_header::RequestHeader as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let subscription_acknowledgements = <Option<
            Vec<super::subscription_acknowledgement::SubscriptionAcknowledgement>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            subscription_acknowledgements,
        })
    }
}
