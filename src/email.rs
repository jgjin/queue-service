// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `email.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(PartialEq,Clone,Default)]
pub struct EmailRequest {
    // message fields
    pub to: ::std::string::String,
    pub subject: ::std::string::String,
    pub body: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EmailRequest {
    fn default() -> &'a EmailRequest {
        <EmailRequest as ::protobuf::Message>::default_instance()
    }
}

impl EmailRequest {
    pub fn new() -> EmailRequest {
        ::std::default::Default::default()
    }

    // string to = 1;


    pub fn get_to(&self) -> &str {
        &self.to
    }
    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::string::String) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.to, ::std::string::String::new())
    }

    // string subject = 2;


    pub fn get_subject(&self) -> &str {
        &self.subject
    }
    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    // string body = 3;


    pub fn get_body(&self) -> &str {
        &self.body
    }
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::string::String) {
        self.body = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::string::String {
        &mut self.body
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.body, ::std::string::String::new())
    }
}

impl ::protobuf::Message for EmailRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.body)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.to);
        }
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        }
        if !self.body.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.body);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.to.is_empty() {
            os.write_string(1, &self.to)?;
        }
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        }
        if !self.body.is_empty() {
            os.write_string(3, &self.body)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EmailRequest {
        EmailRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "to",
                    |m: &EmailRequest| { &m.to },
                    |m: &mut EmailRequest| { &mut m.to },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    |m: &EmailRequest| { &m.subject },
                    |m: &mut EmailRequest| { &mut m.subject },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "body",
                    |m: &EmailRequest| { &m.body },
                    |m: &mut EmailRequest| { &mut m.body },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EmailRequest>(
                    "EmailRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EmailRequest {
        static mut instance: ::protobuf::lazy::Lazy<EmailRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EmailRequest,
        };
        unsafe {
            instance.get(EmailRequest::new)
        }
    }
}

impl ::protobuf::Clear for EmailRequest {
    fn clear(&mut self) {
        self.to.clear();
        self.subject.clear();
        self.body.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EmailRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EmailRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EmailResponse {
    // message fields
    pub status: EmailResponse_Status,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EmailResponse {
    fn default() -> &'a EmailResponse {
        <EmailResponse as ::protobuf::Message>::default_instance()
    }
}

impl EmailResponse {
    pub fn new() -> EmailResponse {
        ::std::default::Default::default()
    }

    // .email.EmailResponse.Status status = 1;


    pub fn get_status(&self) -> EmailResponse_Status {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = EmailResponse_Status::SUCCESS;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: EmailResponse_Status) {
        self.status = v;
    }
}

impl ::protobuf::Message for EmailResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != EmailResponse_Status::SUCCESS {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != EmailResponse_Status::SUCCESS {
            os.write_enum(1, self.status.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EmailResponse {
        EmailResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EmailResponse_Status>>(
                    "status",
                    |m: &EmailResponse| { &m.status },
                    |m: &mut EmailResponse| { &mut m.status },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EmailResponse>(
                    "EmailResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EmailResponse {
        static mut instance: ::protobuf::lazy::Lazy<EmailResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EmailResponse,
        };
        unsafe {
            instance.get(EmailResponse::new)
        }
    }
}

impl ::protobuf::Clear for EmailResponse {
    fn clear(&mut self) {
        self.status = EmailResponse_Status::SUCCESS;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EmailResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EmailResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EmailResponse_Status {
    SUCCESS = 0,
    FAILURE = 1,
}

impl ::protobuf::ProtobufEnum for EmailResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EmailResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(EmailResponse_Status::SUCCESS),
            1 => ::std::option::Option::Some(EmailResponse_Status::FAILURE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EmailResponse_Status] = &[
            EmailResponse_Status::SUCCESS,
            EmailResponse_Status::FAILURE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EmailResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EmailResponse_Status {
}

impl ::std::default::Default for EmailResponse_Status {
    fn default() -> Self {
        EmailResponse_Status::SUCCESS
    }
}

impl ::protobuf::reflect::ProtobufValue for EmailResponse_Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bemail.proto\x12\x05email\"L\n\x0cEmailRequest\x12\x0e\n\x02to\x18\
    \x01\x20\x01(\tR\x02to\x12\x18\n\x07subject\x18\x02\x20\x01(\tR\x07subje\
    ct\x12\x12\n\x04body\x18\x03\x20\x01(\tR\x04body\"h\n\rEmailResponse\x12\
    3\n\x06status\x18\x01\x20\x01(\x0e2\x1b.email.EmailResponse.StatusR\x06s\
    tatus\"\"\n\x06Status\x12\x0b\n\x07SUCCESS\x10\0\x12\x0b\n\x07FAILURE\
    \x10\x012F\n\x0cEmailService\x126\n\tSendEmail\x12\x13.email.EmailReques\
    t\x1a\x14.email.EmailResponseb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
