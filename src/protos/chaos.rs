// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ChaosRequest {
    // message fields
    pub whatever: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChaosRequest {}

impl ChaosRequest {
    pub fn new() -> ChaosRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChaosRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChaosRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChaosRequest,
        };
        unsafe {
            instance.get(ChaosRequest::new)
        }
    }

    // string whatever = 1;

    pub fn clear_whatever(&mut self) {
        self.whatever.clear();
    }

    // Param is passed by value, moved
    pub fn set_whatever(&mut self, v: ::std::string::String) {
        self.whatever = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_whatever(&mut self) -> &mut ::std::string::String {
        &mut self.whatever
    }

    // Take field
    pub fn take_whatever(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.whatever, ::std::string::String::new())
    }

    pub fn get_whatever(&self) -> &str {
        &self.whatever
    }

    fn get_whatever_for_reflect(&self) -> &::std::string::String {
        &self.whatever
    }

    fn mut_whatever_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.whatever
    }
}

impl ::protobuf::Message for ChaosRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.whatever)?;
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
        if !self.whatever.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.whatever);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.whatever.is_empty() {
            os.write_string(1, &self.whatever)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChaosRequest {
    fn new() -> ChaosRequest {
        ChaosRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChaosRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "whatever",
                    ChaosRequest::get_whatever_for_reflect,
                    ChaosRequest::mut_whatever_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChaosRequest>(
                    "ChaosRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChaosRequest {
    fn clear(&mut self) {
        self.clear_whatever();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChaosRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChaosRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChaosResponse {
    // message fields
    pub id: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChaosResponse {}

impl ChaosResponse {
    pub fn new() -> ChaosResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChaosResponse {
        static mut instance: ::protobuf::lazy::Lazy<ChaosResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChaosResponse,
        };
        unsafe {
            instance.get(ChaosResponse::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }
}

impl ::protobuf::Message for ChaosResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChaosResponse {
    fn new() -> ChaosResponse {
        ChaosResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChaosResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    ChaosResponse::get_id_for_reflect,
                    ChaosResponse::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChaosResponse>(
                    "ChaosResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChaosResponse {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChaosResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChaosResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/protos/chaos.proto\"*\n\x0cChaosRequest\x12\x1a\n\x08whatever\
    \x18\x01\x20\x01(\tR\x08whatever\"\x1f\n\rChaosResponse\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\x04R\x02id27\n\x05Chaos\x12.\n\x0bSpreadChaos\x12\r.Ch\
    aosRequest\x1a\x0e.ChaosResponse\"\0J\x9d\x02\n\x06\x12\x04\0\0\x0c\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x06\0\x12\x04\x02\0\x04\x01\n\
    \n\n\x03\x06\0\x01\x12\x03\x02\x08\r\n\x0b\n\x04\x06\0\x02\0\x12\x03\x03\
    \x04=\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x03\x08\x13\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03\x03\x15!\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x03,9\n\
    \n\n\x02\x04\0\x12\x04\x06\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\
    \x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\x04\x18\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x07\x04\x06\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x04\
    \n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x0b\x13\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x07\x16\x17\n\n\n\x02\x04\x01\x12\x04\n\0\x0c\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03\n\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\
    \x04\x12\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0b\x04\n\x17\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x0b\x0b\r\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x10\x11b\x06pro\
    to3\
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
