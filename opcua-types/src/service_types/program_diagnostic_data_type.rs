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
pub struct ProgramDiagnosticDataType {
    pub create_session_id: opcua::types::node_id::NodeId,
    pub create_client_name: opcua::types::string::UAString,
    pub invocation_creation_time: opcua::types::date_time::DateTime,
    pub last_transition_time: opcua::types::date_time::DateTime,
    pub last_method_call: opcua::types::string::UAString,
    pub last_method_session_id: opcua::types::node_id::NodeId,
    pub last_method_input_arguments: Option<Vec<super::argument::Argument>>,
    pub last_method_output_arguments: Option<Vec<super::argument::Argument>>,
    pub last_method_call_time: opcua::types::date_time::DateTime,
    pub last_method_return_status: super::status_result::StatusResult,
}
impl opcua::types::MessageInfo for ProgramDiagnosticDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ProgramDiagnosticDataType {
    #[allow(unused_variables)]
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size = 0usize;
        size += self.create_session_id.byte_len(ctx);
        size += self.create_client_name.byte_len(ctx);
        size += self.invocation_creation_time.byte_len(ctx);
        size += self.last_transition_time.byte_len(ctx);
        size += self.last_method_call.byte_len(ctx);
        size += self.last_method_session_id.byte_len(ctx);
        size += self.last_method_input_arguments.byte_len(ctx);
        size += self.last_method_output_arguments.byte_len(ctx);
        size += self.last_method_call_time.byte_len(ctx);
        size += self.last_method_return_status.byte_len(ctx);
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.create_session_id.encode(stream, ctx)?;
        size += self.create_client_name.encode(stream, ctx)?;
        size += self.invocation_creation_time.encode(stream, ctx)?;
        size += self.last_transition_time.encode(stream, ctx)?;
        size += self.last_method_call.encode(stream, ctx)?;
        size += self.last_method_session_id.encode(stream, ctx)?;
        size += self.last_method_input_arguments.encode(stream, ctx)?;
        size += self.last_method_output_arguments.encode(stream, ctx)?;
        size += self.last_method_call_time.encode(stream, ctx)?;
        size += self.last_method_return_status.encode(stream, ctx)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for ProgramDiagnosticDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read + ?Sized>(
        stream: &mut S,
        ctx: &opcua::types::Context<'_>,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            create_session_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            create_client_name: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            invocation_creation_time: opcua::types::BinaryDecodable::decode(
                stream,
                ctx,
            )?,
            last_transition_time: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            last_method_call: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            last_method_session_id: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            last_method_input_arguments: opcua::types::BinaryDecodable::decode(
                stream,
                ctx,
            )?,
            last_method_output_arguments: opcua::types::BinaryDecodable::decode(
                stream,
                ctx,
            )?,
            last_method_call_time: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            last_method_return_status: opcua::types::BinaryDecodable::decode(
                stream,
                ctx,
            )?,
        })
    }
}
