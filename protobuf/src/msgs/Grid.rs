// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `foxglove/Grid.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foxglove.Grid)
pub struct Grid {
    // message fields
    // @@protoc_insertion_point(field:foxglove.Grid.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:foxglove.Grid.frame_id)
    pub frame_id: ::std::string::String,
    // @@protoc_insertion_point(field:foxglove.Grid.pose)
    pub pose: ::protobuf::MessageField<super::Pose::Pose>,
    // @@protoc_insertion_point(field:foxglove.Grid.column_count)
    pub column_count: u32,
    // @@protoc_insertion_point(field:foxglove.Grid.cell_size)
    pub cell_size: ::protobuf::MessageField<super::Vector2::Vector2>,
    // @@protoc_insertion_point(field:foxglove.Grid.row_stride)
    pub row_stride: u32,
    // @@protoc_insertion_point(field:foxglove.Grid.cell_stride)
    pub cell_stride: u32,
    // @@protoc_insertion_point(field:foxglove.Grid.fields)
    pub fields: ::std::vec::Vec<super::PackedElementField::PackedElementField>,
    // @@protoc_insertion_point(field:foxglove.Grid.data)
    pub data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:foxglove.Grid.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Grid {
    fn default() -> &'a Grid {
        <Grid as ::protobuf::Message>::default_instance()
    }
}

impl Grid {
    pub fn new() -> Grid {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &Grid| { &m.timestamp },
            |m: &mut Grid| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "frame_id",
            |m: &Grid| { &m.frame_id },
            |m: &mut Grid| { &mut m.frame_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Pose::Pose>(
            "pose",
            |m: &Grid| { &m.pose },
            |m: &mut Grid| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "column_count",
            |m: &Grid| { &m.column_count },
            |m: &mut Grid| { &mut m.column_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector2::Vector2>(
            "cell_size",
            |m: &Grid| { &m.cell_size },
            |m: &mut Grid| { &mut m.cell_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "row_stride",
            |m: &Grid| { &m.row_stride },
            |m: &mut Grid| { &mut m.row_stride },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cell_stride",
            |m: &Grid| { &m.cell_stride },
            |m: &mut Grid| { &mut m.cell_stride },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "fields",
            |m: &Grid| { &m.fields },
            |m: &mut Grid| { &mut m.fields },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &Grid| { &m.data },
            |m: &mut Grid| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Grid>(
            "Grid",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Grid {
    const NAME: &'static str = "Grid";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                18 => {
                    self.frame_id = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                37 => {
                    self.column_count = is.read_fixed32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cell_size)?;
                },
                53 => {
                    self.row_stride = is.read_fixed32()?;
                },
                61 => {
                    self.cell_stride = is.read_fixed32()?;
                },
                66 => {
                    self.fields.push(is.read_message()?);
                },
                74 => {
                    self.data = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.frame_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.frame_id);
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.column_count != 0 {
            my_size += 1 + 4;
        }
        if let Some(v) = self.cell_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.row_stride != 0 {
            my_size += 1 + 4;
        }
        if self.cell_stride != 0 {
            my_size += 1 + 4;
        }
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(9, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.frame_id.is_empty() {
            os.write_string(2, &self.frame_id)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.column_count != 0 {
            os.write_fixed32(4, self.column_count)?;
        }
        if let Some(v) = self.cell_size.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.row_stride != 0 {
            os.write_fixed32(6, self.row_stride)?;
        }
        if self.cell_stride != 0 {
            os.write_fixed32(7, self.cell_stride)?;
        }
        for v in &self.fields {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(9, &self.data)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Grid {
        Grid::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.frame_id.clear();
        self.pose.clear();
        self.column_count = 0;
        self.cell_size.clear();
        self.row_stride = 0;
        self.cell_stride = 0;
        self.fields.clear();
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Grid {
        static instance: Grid = Grid {
            timestamp: ::protobuf::MessageField::none(),
            frame_id: ::std::string::String::new(),
            pose: ::protobuf::MessageField::none(),
            column_count: 0,
            cell_size: ::protobuf::MessageField::none(),
            row_stride: 0,
            cell_stride: 0,
            fields: ::std::vec::Vec::new(),
            data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Grid {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Grid").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Grid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Grid {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13foxglove/Grid.proto\x12\x08foxglove\x1a!foxglove/PackedElementFiel\
    d.proto\x1a\x13foxglove/Pose.proto\x1a\x16foxglove/Vector2.proto\x1a\x1f\
    google/protobuf/timestamp.proto\"\xdc\x02\n\x04Grid\x128\n\ttimestamp\
    \x18\x01\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ttimestamp\x12\x19\
    \n\x08frame_id\x18\x02\x20\x01(\tR\x07frameId\x12\"\n\x04pose\x18\x03\
    \x20\x01(\x0b2\x0e.foxglove.PoseR\x04pose\x12!\n\x0ccolumn_count\x18\x04\
    \x20\x01(\x07R\x0bcolumnCount\x12.\n\tcell_size\x18\x05\x20\x01(\x0b2\
    \x11.foxglove.Vector2R\x08cellSize\x12\x1d\n\nrow_stride\x18\x06\x20\x01\
    (\x07R\trowStride\x12\x1f\n\x0bcell_stride\x18\x07\x20\x01(\x07R\ncellSt\
    ride\x124\n\x06fields\x18\x08\x20\x03(\x0b2\x1c.foxglove.PackedElementFi\
    eldR\x06fields\x12\x12\n\x04data\x18\t\x20\x01(\x0cR\x04datab\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::PackedElementField::file_descriptor().clone());
            deps.push(super::Pose::file_descriptor().clone());
            deps.push(super::Vector2::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Grid::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}