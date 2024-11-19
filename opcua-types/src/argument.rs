// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

use std::io::{Read, Write};

use log::error;

use crate::{
    encoding::*, localized_text::LocalizedText, node_id::NodeId, status_code::StatusCode,
    string::UAString, Context,
};

// From OPC UA Part 3 - Address Space Model 1.03 Specification
//
// This Structured DataType defines a Method input or output argument specification. It is for
// example used in the input and output argument Properties for Methods. Its elements are described in
// Table23

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub name: UAString,
    pub data_type: NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub description: LocalizedText,
}

impl BinaryEncodable for Argument {
    fn byte_len(&self, ctx: &crate::Context<'_>) -> usize {
        let mut size = 0;
        size += self.name.byte_len(ctx);
        size += self.data_type.byte_len(ctx);
        size += self.value_rank.byte_len(ctx);
        size += self.array_dimensions.byte_len(ctx);
        size += self.description.byte_len(ctx);
        size
    }

    fn encode<S: Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &Context<'_>,
    ) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream, ctx)?;
        size += self.data_type.encode(stream, ctx)?;
        size += self.value_rank.encode(stream, ctx)?;
        // Encode the array dimensions
        if self.value_rank > 0 {
            if let Some(ref array_dimensions) = self.array_dimensions {
                if self.value_rank as usize != array_dimensions.len() {
                    error!("The array dimensions {} of the Argument should match value rank {} and they don't", array_dimensions.len(), self.value_rank);
                    return Err(StatusCode::BadDataEncodingInvalid.into());
                }
                size += self.array_dimensions.encode(stream, ctx)?;
            } else {
                error!("The array dimensions are expected in the Argument matching value rank {} and they aren't", self.value_rank);
                return Err(StatusCode::BadDataEncodingInvalid.into());
            }
        } else {
            size += write_u32(stream, 0u32)?;
        }

        size += self.description.encode(stream, ctx)?;
        Ok(size)
    }
}

impl BinaryDecodable for Argument {
    fn decode<S: Read + ?Sized>(stream: &mut S, ctx: &Context<'_>) -> EncodingResult<Self> {
        let name = UAString::decode(stream, ctx)?;
        let data_type = NodeId::decode(stream, ctx)?;
        let value_rank = i32::decode(stream, ctx)?;
        // Decode array dimensions
        let array_dimensions: Option<Vec<u32>> = BinaryDecodable::decode(stream, ctx)?;
        if let Some(ref array_dimensions) = array_dimensions {
            if value_rank > 0 && value_rank as usize != array_dimensions.len() {
                error!("The array dimensions {} of the Argument should match value rank {} and they don't", array_dimensions.len(), value_rank);
                return Err(StatusCode::BadDataEncodingInvalid.into());
            }
        }
        let description = LocalizedText::decode(stream, ctx)?;
        Ok(Argument {
            name,
            data_type,
            value_rank,
            array_dimensions,
            description,
        })
    }
}
