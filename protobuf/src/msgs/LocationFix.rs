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

//! Generated file from `foxglove/LocationFix.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foxglove.LocationFix)
pub struct LocationFix {
    // message fields
    // @@protoc_insertion_point(field:foxglove.LocationFix.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:foxglove.LocationFix.frame_id)
    pub frame_id: ::std::string::String,
    // @@protoc_insertion_point(field:foxglove.LocationFix.latitude)
    pub latitude: f64,
    // @@protoc_insertion_point(field:foxglove.LocationFix.longitude)
    pub longitude: f64,
    // @@protoc_insertion_point(field:foxglove.LocationFix.altitude)
    pub altitude: f64,
    // @@protoc_insertion_point(field:foxglove.LocationFix.position_covariance)
    pub position_covariance: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:foxglove.LocationFix.position_covariance_type)
    pub position_covariance_type: ::protobuf::EnumOrUnknown<location_fix::PositionCovarianceType>,
    // special fields
    // @@protoc_insertion_point(special_field:foxglove.LocationFix.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LocationFix {
    fn default() -> &'a LocationFix {
        <LocationFix as ::protobuf::Message>::default_instance()
    }
}

impl LocationFix {
    pub fn new() -> LocationFix {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &LocationFix| { &m.timestamp },
            |m: &mut LocationFix| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "frame_id",
            |m: &LocationFix| { &m.frame_id },
            |m: &mut LocationFix| { &mut m.frame_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "latitude",
            |m: &LocationFix| { &m.latitude },
            |m: &mut LocationFix| { &mut m.latitude },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "longitude",
            |m: &LocationFix| { &m.longitude },
            |m: &mut LocationFix| { &mut m.longitude },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "altitude",
            |m: &LocationFix| { &m.altitude },
            |m: &mut LocationFix| { &mut m.altitude },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "position_covariance",
            |m: &LocationFix| { &m.position_covariance },
            |m: &mut LocationFix| { &mut m.position_covariance },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "position_covariance_type",
            |m: &LocationFix| { &m.position_covariance_type },
            |m: &mut LocationFix| { &mut m.position_covariance_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LocationFix>(
            "LocationFix",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LocationFix {
    const NAME: &'static str = "LocationFix";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                58 => {
                    self.frame_id = is.read_string()?;
                },
                9 => {
                    self.latitude = is.read_double()?;
                },
                17 => {
                    self.longitude = is.read_double()?;
                },
                25 => {
                    self.altitude = is.read_double()?;
                },
                34 => {
                    is.read_repeated_packed_double_into(&mut self.position_covariance)?;
                },
                33 => {
                    self.position_covariance.push(is.read_double()?);
                },
                40 => {
                    self.position_covariance_type = is.read_enum_or_unknown()?;
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
            my_size += ::protobuf::rt::string_size(7, &self.frame_id);
        }
        if self.latitude != 0. {
            my_size += 1 + 8;
        }
        if self.longitude != 0. {
            my_size += 1 + 8;
        }
        if self.altitude != 0. {
            my_size += 1 + 8;
        }
        my_size += 9 * self.position_covariance.len() as u64;
        if self.position_covariance_type != ::protobuf::EnumOrUnknown::new(location_fix::PositionCovarianceType::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(5, self.position_covariance_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if !self.frame_id.is_empty() {
            os.write_string(7, &self.frame_id)?;
        }
        if self.latitude != 0. {
            os.write_double(1, self.latitude)?;
        }
        if self.longitude != 0. {
            os.write_double(2, self.longitude)?;
        }
        if self.altitude != 0. {
            os.write_double(3, self.altitude)?;
        }
        for v in &self.position_covariance {
            os.write_double(4, *v)?;
        };
        if self.position_covariance_type != ::protobuf::EnumOrUnknown::new(location_fix::PositionCovarianceType::UNKNOWN) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.position_covariance_type))?;
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

    fn new() -> LocationFix {
        LocationFix::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.frame_id.clear();
        self.latitude = 0.;
        self.longitude = 0.;
        self.altitude = 0.;
        self.position_covariance.clear();
        self.position_covariance_type = ::protobuf::EnumOrUnknown::new(location_fix::PositionCovarianceType::UNKNOWN);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LocationFix {
        static instance: LocationFix = LocationFix {
            timestamp: ::protobuf::MessageField::none(),
            frame_id: ::std::string::String::new(),
            latitude: 0.,
            longitude: 0.,
            altitude: 0.,
            position_covariance: ::std::vec::Vec::new(),
            position_covariance_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LocationFix {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LocationFix").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LocationFix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocationFix {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LocationFix`
pub mod location_fix {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:foxglove.LocationFix.PositionCovarianceType)
    pub enum PositionCovarianceType {
        // @@protoc_insertion_point(enum_value:foxglove.LocationFix.PositionCovarianceType.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:foxglove.LocationFix.PositionCovarianceType.APPROXIMATED)
        APPROXIMATED = 1,
        // @@protoc_insertion_point(enum_value:foxglove.LocationFix.PositionCovarianceType.DIAGONAL_KNOWN)
        DIAGONAL_KNOWN = 2,
        // @@protoc_insertion_point(enum_value:foxglove.LocationFix.PositionCovarianceType.KNOWN)
        KNOWN = 3,
    }

    impl ::protobuf::Enum for PositionCovarianceType {
        const NAME: &'static str = "PositionCovarianceType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<PositionCovarianceType> {
            match value {
                0 => ::std::option::Option::Some(PositionCovarianceType::UNKNOWN),
                1 => ::std::option::Option::Some(PositionCovarianceType::APPROXIMATED),
                2 => ::std::option::Option::Some(PositionCovarianceType::DIAGONAL_KNOWN),
                3 => ::std::option::Option::Some(PositionCovarianceType::KNOWN),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [PositionCovarianceType] = &[
            PositionCovarianceType::UNKNOWN,
            PositionCovarianceType::APPROXIMATED,
            PositionCovarianceType::DIAGONAL_KNOWN,
            PositionCovarianceType::KNOWN,
        ];
    }

    impl ::protobuf::EnumFull for PositionCovarianceType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("LocationFix.PositionCovarianceType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for PositionCovarianceType {
        fn default() -> Self {
            PositionCovarianceType::UNKNOWN
        }
    }

    impl PositionCovarianceType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PositionCovarianceType>("LocationFix.PositionCovarianceType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1afoxglove/LocationFix.proto\x12\x08foxglove\x1a\x1fgoogle/protobuf/\
    timestamp.proto\"\xa9\x03\n\x0bLocationFix\x128\n\ttimestamp\x18\x06\x20\
    \x01(\x0b2\x1a.google.protobuf.TimestampR\ttimestamp\x12\x19\n\x08frame_\
    id\x18\x07\x20\x01(\tR\x07frameId\x12\x1a\n\x08latitude\x18\x01\x20\x01(\
    \x01R\x08latitude\x12\x1c\n\tlongitude\x18\x02\x20\x01(\x01R\tlongitude\
    \x12\x1a\n\x08altitude\x18\x03\x20\x01(\x01R\x08altitude\x12/\n\x13posit\
    ion_covariance\x18\x04\x20\x03(\x01R\x12positionCovariance\x12f\n\x18pos\
    ition_covariance_type\x18\x05\x20\x01(\x0e2,.foxglove.LocationFix.Positi\
    onCovarianceTypeR\x16positionCovarianceType\"V\n\x16PositionCovarianceTy\
    pe\x12\x0b\n\x07UNKNOWN\x10\0\x12\x10\n\x0cAPPROXIMATED\x10\x01\x12\x12\
    \n\x0eDIAGONAL_KNOWN\x10\x02\x12\t\n\x05KNOWN\x10\x03b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LocationFix::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(location_fix::PositionCovarianceType::generated_enum_descriptor_data());
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
