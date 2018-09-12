// This file is generated by rust-protobuf 2.0.4. Do not edit
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
pub struct Block {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::blockHeader::BlockHeader>,
    pub txs: ::protobuf::RepeatedField<super::tx::SignedTx>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    // .BlockHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::blockHeader::BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::blockHeader::BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::blockHeader::BlockHeader {
        self.header.take().unwrap_or_else(|| super::blockHeader::BlockHeader::new())
    }

    pub fn get_header(&self) -> &super::blockHeader::BlockHeader {
        self.header.as_ref().unwrap_or_else(|| super::blockHeader::BlockHeader::default_instance())
    }

    // repeated .SignedTx txs = 2;

    pub fn clear_txs(&mut self) {
        self.txs.clear();
    }

    // Param is passed by value, moved
    pub fn set_txs(&mut self, v: ::protobuf::RepeatedField<super::tx::SignedTx>) {
        self.txs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txs(&mut self) -> &mut ::protobuf::RepeatedField<super::tx::SignedTx> {
        &mut self.txs
    }

    // Take field
    pub fn take_txs(&mut self) -> ::protobuf::RepeatedField<super::tx::SignedTx> {
        ::std::mem::replace(&mut self.txs, ::protobuf::RepeatedField::new())
    }

    pub fn get_txs(&self) -> &[super::tx::SignedTx] {
        &self.txs
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txs)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.txs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.txs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockHeader::BlockHeader>>(
                    "header",
                    |m: &Block| { &m.header },
                    |m: &mut Block| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tx::SignedTx>>(
                    "txs",
                    |m: &Block| { &m.txs },
                    |m: &mut Block| { &mut m.txs },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_txs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenesisBlock {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::blockHeader::GenesisHeader>,
    pub txs: ::protobuf::RepeatedField<super::tx::GenesisSignedTx>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl GenesisBlock {
    pub fn new() -> GenesisBlock {
        ::std::default::Default::default()
    }

    // .GenesisHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::blockHeader::GenesisHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::blockHeader::GenesisHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::blockHeader::GenesisHeader {
        self.header.take().unwrap_or_else(|| super::blockHeader::GenesisHeader::new())
    }

    pub fn get_header(&self) -> &super::blockHeader::GenesisHeader {
        self.header.as_ref().unwrap_or_else(|| super::blockHeader::GenesisHeader::default_instance())
    }

    // repeated .GenesisSignedTx txs = 2;

    pub fn clear_txs(&mut self) {
        self.txs.clear();
    }

    // Param is passed by value, moved
    pub fn set_txs(&mut self, v: ::protobuf::RepeatedField<super::tx::GenesisSignedTx>) {
        self.txs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txs(&mut self) -> &mut ::protobuf::RepeatedField<super::tx::GenesisSignedTx> {
        &mut self.txs
    }

    // Take field
    pub fn take_txs(&mut self) -> ::protobuf::RepeatedField<super::tx::GenesisSignedTx> {
        ::std::mem::replace(&mut self.txs, ::protobuf::RepeatedField::new())
    }

    pub fn get_txs(&self) -> &[super::tx::GenesisSignedTx] {
        &self.txs
    }
}

impl ::protobuf::Message for GenesisBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txs)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.txs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.txs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> GenesisBlock {
        GenesisBlock::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockHeader::GenesisHeader>>(
                    "header",
                    |m: &GenesisBlock| { &m.header },
                    |m: &mut GenesisBlock| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tx::GenesisSignedTx>>(
                    "txs",
                    |m: &GenesisBlock| { &m.txs },
                    |m: &mut GenesisBlock| { &mut m.txs },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenesisBlock>(
                    "GenesisBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GenesisBlock {
        static mut instance: ::protobuf::lazy::Lazy<GenesisBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenesisBlock,
        };
        unsafe {
            instance.get(GenesisBlock::new)
        }
    }
}

impl ::protobuf::Clear for GenesisBlock {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_txs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenesisBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenesisBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockDB {
    // message fields
    pub height: u32,
    pub header: ::protobuf::SingularPtrField<super::blockHeader::BlockHeader>,
    pub fileNumber: u32,
    pub offset: u32,
    pub length: u32,
    pub tEMA: f64,
    pub pEMA: f64,
    pub nextDifficulty: f64,
    pub totalWork: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl BlockDB {
    pub fn new() -> BlockDB {
        ::std::default::Default::default()
    }

    // uint32 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = v;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    // .BlockHeader header = 2;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::blockHeader::BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::blockHeader::BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::blockHeader::BlockHeader {
        self.header.take().unwrap_or_else(|| super::blockHeader::BlockHeader::new())
    }

    pub fn get_header(&self) -> &super::blockHeader::BlockHeader {
        self.header.as_ref().unwrap_or_else(|| super::blockHeader::BlockHeader::default_instance())
    }

    // uint32 fileNumber = 3;

    pub fn clear_fileNumber(&mut self) {
        self.fileNumber = 0;
    }

    // Param is passed by value, moved
    pub fn set_fileNumber(&mut self, v: u32) {
        self.fileNumber = v;
    }

    pub fn get_fileNumber(&self) -> u32 {
        self.fileNumber
    }

    // uint32 offset = 4;

    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u32) {
        self.offset = v;
    }

    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    // uint32 length = 5;

    pub fn clear_length(&mut self) {
        self.length = 0;
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = v;
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    // double tEMA = 6;

    pub fn clear_tEMA(&mut self) {
        self.tEMA = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tEMA(&mut self, v: f64) {
        self.tEMA = v;
    }

    pub fn get_tEMA(&self) -> f64 {
        self.tEMA
    }

    // double pEMA = 7;

    pub fn clear_pEMA(&mut self) {
        self.pEMA = 0.;
    }

    // Param is passed by value, moved
    pub fn set_pEMA(&mut self, v: f64) {
        self.pEMA = v;
    }

    pub fn get_pEMA(&self) -> f64 {
        self.pEMA
    }

    // double nextDifficulty = 8;

    pub fn clear_nextDifficulty(&mut self) {
        self.nextDifficulty = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nextDifficulty(&mut self, v: f64) {
        self.nextDifficulty = v;
    }

    pub fn get_nextDifficulty(&self) -> f64 {
        self.nextDifficulty
    }

    // double totalWork = 9;

    pub fn clear_totalWork(&mut self) {
        self.totalWork = 0.;
    }

    // Param is passed by value, moved
    pub fn set_totalWork(&mut self, v: f64) {
        self.totalWork = v;
    }

    pub fn get_totalWork(&self) -> f64 {
        self.totalWork
    }
}

impl ::protobuf::Message for BlockDB {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    let tmp = is.read_uint32()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fileNumber = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.offset = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.length = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tEMA = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.pEMA = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nextDifficulty = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.totalWork = tmp;
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
        if true {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if true {
            my_size += ::protobuf::rt::value_size(3, self.fileNumber, ::protobuf::wire_format::WireTypeVarint);
        }
        if true {
            my_size += ::protobuf::rt::value_size(4, self.offset, ::protobuf::wire_format::WireTypeVarint);
        }
        if true {
            my_size += ::protobuf::rt::value_size(5, self.length, ::protobuf::wire_format::WireTypeVarint);
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if true {
            os.write_uint32(1, self.height)?;
        }
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if true {
            os.write_uint32(3, self.fileNumber)?;
        }
        if true {
            os.write_uint32(4, self.offset)?;
        }
        if true {
            os.write_uint32(5, self.length)?;
        }
        if true {
            os.write_double(6, self.tEMA)?;
        }
        if true {
            os.write_double(7, self.pEMA)?;
        }
        if true {
            os.write_double(8, self.nextDifficulty)?;
        }
        if true {
            os.write_double(9, self.totalWork)?;
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

    fn new() -> BlockDB {
        BlockDB::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "height",
                    |m: &BlockDB| { &m.height },
                    |m: &mut BlockDB| { &mut m.height },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockHeader::BlockHeader>>(
                    "header",
                    |m: &BlockDB| { &m.header },
                    |m: &mut BlockDB| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fileNumber",
                    |m: &BlockDB| { &m.fileNumber },
                    |m: &mut BlockDB| { &mut m.fileNumber },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "offset",
                    |m: &BlockDB| { &m.offset },
                    |m: &mut BlockDB| { &mut m.offset },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "length",
                    |m: &BlockDB| { &m.length },
                    |m: &mut BlockDB| { &mut m.length },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "tEMA",
                    |m: &BlockDB| { &m.tEMA },
                    |m: &mut BlockDB| { &mut m.tEMA },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "pEMA",
                    |m: &BlockDB| { &m.pEMA },
                    |m: &mut BlockDB| { &mut m.pEMA },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "nextDifficulty",
                    |m: &BlockDB| { &m.nextDifficulty },
                    |m: &mut BlockDB| { &mut m.nextDifficulty },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "totalWork",
                    |m: &BlockDB| { &m.totalWork },
                    |m: &mut BlockDB| { &mut m.totalWork },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockDB>(
                    "BlockDB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockDB {
        static mut instance: ::protobuf::lazy::Lazy<BlockDB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockDB,
        };
        unsafe {
            instance.get(BlockDB::new)
        }
    }
}

impl ::protobuf::Clear for BlockDB {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_header();
        self.clear_fileNumber();
        self.clear_offset();
        self.clear_length();
        self.clear_tEMA();
        self.clear_pEMA();
        self.clear_nextDifficulty();
        self.clear_totalWork();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockDB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockDB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bblock.proto\x1a\x08tx.proto\x1a\x11blockHeader.proto\"J\n\x05Block\
    \x12$\n\x06header\x18\x01\x20\x01(\x0b2\x0c.BlockHeaderR\x06header\x12\
    \x1b\n\x03txs\x18\x02\x20\x03(\x0b2\t.SignedTxR\x03txs\"Z\n\x0cGenesisBl\
    ock\x12&\n\x06header\x18\x01\x20\x01(\x0b2\x0e.GenesisHeaderR\x06header\
    \x12\"\n\x03txs\x18\x02\x20\x03(\x0b2\x10.GenesisSignedTxR\x03txs\"\x85\
    \x02\n\x07BlockDB\x12\x16\n\x06height\x18\x01\x20\x01(\rR\x06height\x12$\
    \n\x06header\x18\x02\x20\x01(\x0b2\x0c.BlockHeaderR\x06header\x12\x1e\n\
    \nfileNumber\x18\x03\x20\x01(\rR\nfileNumber\x12\x16\n\x06offset\x18\x04\
    \x20\x01(\rR\x06offset\x12\x16\n\x06length\x18\x05\x20\x01(\rR\x06length\
    \x12\x12\n\x04tEMA\x18\x06\x20\x01(\x01R\x04tEMA\x12\x12\n\x04pEMA\x18\
    \x07\x20\x01(\x01R\x04pEMA\x12&\n\x0enextDifficulty\x18\x08\x20\x01(\x01\
    R\x0enextDifficulty\x12\x1c\n\ttotalWork\x18\t\x20\x01(\x01R\ttotalWorkb\
    \x06proto3\
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
