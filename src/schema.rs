// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct TableInfo {
    // message fields
    table_id: ::std::option::Option<i64>,
    columns: ::protobuf::RepeatedField<ColumnInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TableInfo {}

impl TableInfo {
    pub fn new() -> TableInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TableInfo {
        static mut instance: ::protobuf::lazy::Lazy<TableInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TableInfo,
        };
        unsafe {
            instance.get(|| {
                TableInfo {
                    table_id: ::std::option::Option::None,
                    columns: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i64 {
        self.table_id.unwrap_or(0)
    }

    // repeated .tipb.ColumnInfo columns = 2;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<ColumnInfo>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<ColumnInfo> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[ColumnInfo] {
        &self.columns
    }
}

impl ::protobuf::Message for TableInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.columns.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            try!(os.write_int64(1, v));
        };
        for v in self.columns.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TableInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TableInfo {
    fn new() -> TableInfo {
        TableInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<TableInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "table_id",
                    TableInfo::has_table_id,
                    TableInfo::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    TableInfo::get_columns,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TableInfo>(
                    "TableInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TableInfo {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_columns();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TableInfo {
    fn eq(&self, other: &TableInfo) -> bool {
        self.table_id == other.table_id &&
        self.columns == other.columns &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TableInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ColumnInfo {
    // message fields
    column_id: ::std::option::Option<i64>,
    tp: ::std::option::Option<i32>,
    collation: ::std::option::Option<i32>,
    columnLen: ::std::option::Option<i32>,
    decimal: ::std::option::Option<i32>,
    flag: ::std::option::Option<i32>,
    elems: ::protobuf::RepeatedField<::std::string::String>,
    pk_handle: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnInfo {}

impl ColumnInfo {
    pub fn new() -> ColumnInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnInfo {
        static mut instance: ::protobuf::lazy::Lazy<ColumnInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnInfo,
        };
        unsafe {
            instance.get(|| {
                ColumnInfo {
                    column_id: ::std::option::Option::None,
                    tp: ::std::option::Option::None,
                    collation: ::std::option::Option::None,
                    columnLen: ::std::option::Option::None,
                    decimal: ::std::option::Option::None,
                    flag: ::std::option::Option::None,
                    elems: ::protobuf::RepeatedField::new(),
                    pk_handle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 column_id = 1;

    pub fn clear_column_id(&mut self) {
        self.column_id = ::std::option::Option::None;
    }

    pub fn has_column_id(&self) -> bool {
        self.column_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_column_id(&mut self, v: i64) {
        self.column_id = ::std::option::Option::Some(v);
    }

    pub fn get_column_id(&self) -> i64 {
        self.column_id.unwrap_or(0)
    }

    // optional int32 tp = 2;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: i32) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> i32 {
        self.tp.unwrap_or(0)
    }

    // optional int32 collation = 3;

    pub fn clear_collation(&mut self) {
        self.collation = ::std::option::Option::None;
    }

    pub fn has_collation(&self) -> bool {
        self.collation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collation(&mut self, v: i32) {
        self.collation = ::std::option::Option::Some(v);
    }

    pub fn get_collation(&self) -> i32 {
        self.collation.unwrap_or(0)
    }

    // optional int32 columnLen = 4;

    pub fn clear_columnLen(&mut self) {
        self.columnLen = ::std::option::Option::None;
    }

    pub fn has_columnLen(&self) -> bool {
        self.columnLen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_columnLen(&mut self, v: i32) {
        self.columnLen = ::std::option::Option::Some(v);
    }

    pub fn get_columnLen(&self) -> i32 {
        self.columnLen.unwrap_or(0)
    }

    // optional int32 decimal = 5;

    pub fn clear_decimal(&mut self) {
        self.decimal = ::std::option::Option::None;
    }

    pub fn has_decimal(&self) -> bool {
        self.decimal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decimal(&mut self, v: i32) {
        self.decimal = ::std::option::Option::Some(v);
    }

    pub fn get_decimal(&self) -> i32 {
        self.decimal.unwrap_or(0)
    }

    // optional int32 flag = 6;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: i32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> i32 {
        self.flag.unwrap_or(0)
    }

    // repeated string elems = 7;

    pub fn clear_elems(&mut self) {
        self.elems.clear();
    }

    // Param is passed by value, moved
    pub fn set_elems(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.elems = v;
    }

    // Mutable pointer to the field.
    pub fn mut_elems(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.elems
    }

    // Take field
    pub fn take_elems(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.elems, ::protobuf::RepeatedField::new())
    }

    pub fn get_elems(&self) -> &[::std::string::String] {
        &self.elems
    }

    // optional bool pk_handle = 21;

    pub fn clear_pk_handle(&mut self) {
        self.pk_handle = ::std::option::Option::None;
    }

    pub fn has_pk_handle(&self) -> bool {
        self.pk_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pk_handle(&mut self, v: bool) {
        self.pk_handle = ::std::option::Option::Some(v);
    }

    pub fn get_pk_handle(&self) -> bool {
        self.pk_handle.unwrap_or(false)
    }
}

impl ::protobuf::Message for ColumnInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.column_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.tp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.collation = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.columnLen = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.decimal = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.flag = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.elems));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.pk_handle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.column_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.tp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.collation.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.columnLen.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.decimal.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.flag.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.elems.iter() {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        if self.pk_handle.is_some() {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.column_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.tp {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.collation {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.columnLen {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.decimal {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.flag {
            try!(os.write_int32(6, v));
        };
        for v in self.elems.iter() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.pk_handle {
            try!(os.write_bool(21, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ColumnInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ColumnInfo {
    fn new() -> ColumnInfo {
        ColumnInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "column_id",
                    ColumnInfo::has_column_id,
                    ColumnInfo::get_column_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "tp",
                    ColumnInfo::has_tp,
                    ColumnInfo::get_tp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "collation",
                    ColumnInfo::has_collation,
                    ColumnInfo::get_collation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "columnLen",
                    ColumnInfo::has_columnLen,
                    ColumnInfo::get_columnLen,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "decimal",
                    ColumnInfo::has_decimal,
                    ColumnInfo::get_decimal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "flag",
                    ColumnInfo::has_flag,
                    ColumnInfo::get_flag,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "elems",
                    ColumnInfo::get_elems,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "pk_handle",
                    ColumnInfo::has_pk_handle,
                    ColumnInfo::get_pk_handle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnInfo>(
                    "ColumnInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnInfo {
    fn clear(&mut self) {
        self.clear_column_id();
        self.clear_tp();
        self.clear_collation();
        self.clear_columnLen();
        self.clear_decimal();
        self.clear_flag();
        self.clear_elems();
        self.clear_pk_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ColumnInfo {
    fn eq(&self, other: &ColumnInfo) -> bool {
        self.column_id == other.column_id &&
        self.tp == other.tp &&
        self.collation == other.collation &&
        self.columnLen == other.columnLen &&
        self.decimal == other.decimal &&
        self.flag == other.flag &&
        self.elems == other.elems &&
        self.pk_handle == other.pk_handle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ColumnInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IndexInfo {
    // message fields
    table_id: ::std::option::Option<i64>,
    index_id: ::std::option::Option<i64>,
    columns: ::protobuf::RepeatedField<ColumnInfo>,
    unique: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IndexInfo {}

impl IndexInfo {
    pub fn new() -> IndexInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IndexInfo {
        static mut instance: ::protobuf::lazy::Lazy<IndexInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IndexInfo,
        };
        unsafe {
            instance.get(|| {
                IndexInfo {
                    table_id: ::std::option::Option::None,
                    index_id: ::std::option::Option::None,
                    columns: ::protobuf::RepeatedField::new(),
                    unique: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i64 {
        self.table_id.unwrap_or(0)
    }

    // optional int64 index_id = 2;

    pub fn clear_index_id(&mut self) {
        self.index_id = ::std::option::Option::None;
    }

    pub fn has_index_id(&self) -> bool {
        self.index_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index_id(&mut self, v: i64) {
        self.index_id = ::std::option::Option::Some(v);
    }

    pub fn get_index_id(&self) -> i64 {
        self.index_id.unwrap_or(0)
    }

    // repeated .tipb.ColumnInfo columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<ColumnInfo>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<ColumnInfo> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[ColumnInfo] {
        &self.columns
    }

    // optional bool unique = 4;

    pub fn clear_unique(&mut self) {
        self.unique = ::std::option::Option::None;
    }

    pub fn has_unique(&self) -> bool {
        self.unique.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unique(&mut self, v: bool) {
        self.unique = ::std::option::Option::Some(v);
    }

    pub fn get_unique(&self) -> bool {
        self.unique.unwrap_or(false)
    }
}

impl ::protobuf::Message for IndexInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.index_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.unique = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.index_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.columns.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.unique.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.index_id {
            try!(os.write_int64(2, v));
        };
        for v in self.columns.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unique {
            try!(os.write_bool(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IndexInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IndexInfo {
    fn new() -> IndexInfo {
        IndexInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<IndexInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "table_id",
                    IndexInfo::has_table_id,
                    IndexInfo::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "index_id",
                    IndexInfo::has_index_id,
                    IndexInfo::get_index_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    IndexInfo::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "unique",
                    IndexInfo::has_unique,
                    IndexInfo::get_unique,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IndexInfo>(
                    "IndexInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IndexInfo {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_index_id();
        self.clear_columns();
        self.clear_unique();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IndexInfo {
    fn eq(&self, other: &IndexInfo) -> bool {
        self.table_id == other.table_id &&
        self.index_id == other.index_id &&
        self.columns == other.columns &&
        self.unique == other.unique &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IndexInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04,
    0x74, 0x69, 0x70, 0x62, 0x22, 0x40, 0x0a, 0x09, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x03, 0x12, 0x21, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6c, 0x75,
    0x6d, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x92, 0x01, 0x0a, 0x0a, 0x43, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x70, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x4c, 0x65, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65,
    0x63, 0x69, 0x6d, 0x61, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x66,
    0x6c, 0x61, 0x67, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x6c, 0x65,
    0x6d, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x6b, 0x5f, 0x68,
    0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x15, 0x20, 0x01, 0x28, 0x08, 0x22, 0x62, 0x0a, 0x09, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x10, 0x0a, 0x08, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x21, 0x0a, 0x07,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x0e, 0x0a, 0x06, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x42,
    0x19, 0x0a, 0x15, 0x63, 0x6f, 0x6d, 0x2e, 0x70, 0x69, 0x6e, 0x67, 0x63, 0x61, 0x70, 0x2e, 0x74,
    0x69, 0x64, 0x62, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x50, 0x01, 0x4a, 0xfc, 0x09, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x1c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1d,
    0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03,
    0x05, 0x16, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x09, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x09, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x09, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x15,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d,
    0x23, 0x24, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1e, 0x22,
    0x0d, 0x20, 0x4d, 0x79, 0x53, 0x51, 0x4c, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x0f, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x17, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x10, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x11, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x11, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x11, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x11, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x11, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x12,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x12, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x12, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x12, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x12, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x06, 0x12, 0x03, 0x13, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x13, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x13, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x13, 0x18,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x13, 0x20, 0x21, 0x0a,
    0x34, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x14, 0x08, 0x25, 0x22, 0x27, 0x20, 0x50,
    0x4b, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x6f, 0x77, 0x20, 0x68, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x14, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x14, 0x16, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x14, 0x22, 0x24, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x17, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x17, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x18,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x19, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x17,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x1a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x1a, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x1a, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20,
];

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
