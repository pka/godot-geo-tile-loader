// This file is generated by rust-protobuf 3.3.0.
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

//! Generated file from `vector_tile.proto`
// Generated for lite runtime

use protobuf::Message;
use godot::prelude::*;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:vector_tile.Tile)
#[derive(GodotClass)]
#[class(init, rename=MvtTile)]
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Tile {
    // message fields
    // @@protoc_insertion_point(field:vector_tile.Tile.layers)
    pub layers: ::std::vec::Vec<tile::Layer>,
    // special fields
    // @@protoc_insertion_point(special_field:vector_tile.Tile.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Tile {
    fn default() -> &'a Tile {
        <Tile as ::protobuf::Message>::default_instance()
    }
}

#[godot_api]
impl Tile {
    pub fn new() -> Tile {
        ::std::default::Default::default()
    }
    #[func]
    pub fn layers(&self) -> Array<Gd<tile::Layer>> {
        Array::from_iter(self.layers.iter().map(|el| Gd::new(el.clone())))
    }
    #[func]
    /// Read Tile from byte array
    pub fn read(bytes: PackedByteArray) -> Gd<Tile> {
        let tile = Tile::parse_from_bytes(bytes.as_slice()).unwrap();
        Gd::new(tile)
    }
    /// Load Tile from file
    #[func]
    pub fn load(path: GodotString) -> Gd<Tile> {
        let path = String::from_iter(path.chars_checked());
        let bytes = std::fs::read(&path).unwrap();
        let tile = Tile::parse_from_bytes(&bytes).unwrap();
        Gd::new(tile)
    }
}

impl ::protobuf::Message for Tile {
    const NAME: &'static str = "Tile";

    fn is_initialized(&self) -> bool {
        for v in &self.layers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.layers.push(is.read_message()?);
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
        for value in &self.layers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.layers {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Tile {
        Tile::new()
    }

    fn clear(&mut self) {
        self.layers.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Tile {
        static instance: Tile = Tile {
            layers: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

/// Nested message and enums of message `Tile`
pub mod tile {
    use crate::mvt_commands::{CommandInteger, ParameterInteger};
    use godot::prelude::*;

    // @@protoc_insertion_point(message:vector_tile.Tile.Value)
    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(GodotClass)]
    #[class(init,rename=MvtValue)]
    pub struct Value {
        // message fields
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.string_value)
        pub string_value: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.float_value)
        pub float_value: ::std::option::Option<f32>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.double_value)
        pub double_value: ::std::option::Option<f64>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.int_value)
        pub int_value: ::std::option::Option<i64>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.uint_value)
        pub uint_value: ::std::option::Option<u64>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.sint_value)
        pub sint_value: ::std::option::Option<i64>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Value.bool_value)
        pub bool_value: ::std::option::Option<bool>,
        // special fields
        // @@protoc_insertion_point(special_field:vector_tile.Tile.Value.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Value {
        fn default() -> &'a Value {
            <Value as ::protobuf::Message>::default_instance()
        }
    }

    #[godot_api]
    impl Value {
        pub fn new() -> Value {
            ::std::default::Default::default()
        }

        // optional string string_value = 1;

        pub fn string_value_str(&self) -> &str {
            match self.string_value.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        #[func]
        pub fn string_value(&self) -> GodotString {
            GodotString::from(self.string_value_str())
        }

        #[func]
        pub fn clear_string_value(&mut self) {
            self.string_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_string_value(&self) -> bool {
            self.string_value.is_some()
        }

        // Param is passed by value, moved
        pub fn set_string_value_string(&mut self, v: ::std::string::String) {
            self.string_value = ::std::option::Option::Some(v);
        }

        #[func]
        pub fn set_string_value(&mut self, v: GodotString) {
            self.set_string_value_string(v.into());
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
            if self.string_value.is_none() {
                self.string_value = ::std::option::Option::Some(::std::string::String::new());
            }
            self.string_value.as_mut().unwrap()
        }

        // Take field
        pub fn take_string_value(&mut self) -> ::std::string::String {
            self.string_value.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional float float_value = 2;

        #[func]
        pub fn float_value(&self) -> f32 {
            self.float_value.unwrap_or(0.)
        }

        #[func]
        pub fn clear_float_value(&mut self) {
            self.float_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_float_value(&self) -> bool {
            self.float_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_float_value(&mut self, v: f32) {
            self.float_value = ::std::option::Option::Some(v);
        }

        // optional double double_value = 3;

        #[func]
        pub fn double_value(&self) -> f64 {
            self.double_value.unwrap_or(0.)
        }

        #[func]
        pub fn clear_double_value(&mut self) {
            self.double_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_double_value(&self) -> bool {
            self.double_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_double_value(&mut self, v: f64) {
            self.double_value = ::std::option::Option::Some(v);
        }

        // optional int64 int_value = 4;

        #[func]
        pub fn int_value(&self) -> i64 {
            self.int_value.unwrap_or(0)
        }

        #[func]
        pub fn clear_int_value(&mut self) {
            self.int_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_int_value(&self) -> bool {
            self.int_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_int_value(&mut self, v: i64) {
            self.int_value = ::std::option::Option::Some(v);
        }

        // optional uint64 uint_value = 5;

        #[func]
        pub fn uint_value(&self) -> u64 {
            self.uint_value.unwrap_or(0)
        }

        #[func]
        pub fn clear_uint_value(&mut self) {
            self.uint_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_uint_value(&self) -> bool {
            self.uint_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_uint_value(&mut self, v: u64) {
            self.uint_value = ::std::option::Option::Some(v);
        }

        // optional sint64 sint_value = 6;

        #[func]
        pub fn sint_value(&self) -> i64 {
            self.sint_value.unwrap_or(0)
        }

        #[func]
        pub fn clear_sint_value(&mut self) {
            self.sint_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_sint_value(&self) -> bool {
            self.sint_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_sint_value(&mut self, v: i64) {
            self.sint_value = ::std::option::Option::Some(v);
        }

        // optional bool bool_value = 7;

        #[func]
        pub fn bool_value(&self) -> bool {
            self.bool_value.unwrap_or(false)
        }

        #[func]
        pub fn clear_bool_value(&mut self) {
            self.bool_value = ::std::option::Option::None;
        }

        #[func]
        pub fn has_bool_value(&self) -> bool {
            self.bool_value.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_bool_value(&mut self, v: bool) {
            self.bool_value = ::std::option::Option::Some(v);
        }
    }

    impl ::protobuf::Message for Value {
        const NAME: &'static str = "Value";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.string_value = ::std::option::Option::Some(is.read_string()?);
                    },
                    21 => {
                        self.float_value = ::std::option::Option::Some(is.read_float()?);
                    },
                    25 => {
                        self.double_value = ::std::option::Option::Some(is.read_double()?);
                    },
                    32 => {
                        self.int_value = ::std::option::Option::Some(is.read_int64()?);
                    },
                    40 => {
                        self.uint_value = ::std::option::Option::Some(is.read_uint64()?);
                    },
                    48 => {
                        self.sint_value = ::std::option::Option::Some(is.read_sint64()?);
                    },
                    56 => {
                        self.bool_value = ::std::option::Option::Some(is.read_bool()?);
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
            if let Some(v) = self.string_value.as_ref() {
                my_size += ::protobuf::rt::string_size(1, &v);
            }
            if let Some(v) = self.float_value {
                my_size += 1 + 4;
            }
            if let Some(v) = self.double_value {
                my_size += 1 + 8;
            }
            if let Some(v) = self.int_value {
                my_size += ::protobuf::rt::int64_size(4, v);
            }
            if let Some(v) = self.uint_value {
                my_size += ::protobuf::rt::uint64_size(5, v);
            }
            if let Some(v) = self.sint_value {
                my_size += ::protobuf::rt::sint64_size(6, v);
            }
            if let Some(v) = self.bool_value {
                my_size += 1 + 1;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if let Some(v) = self.string_value.as_ref() {
                os.write_string(1, v)?;
            }
            if let Some(v) = self.float_value {
                os.write_float(2, v)?;
            }
            if let Some(v) = self.double_value {
                os.write_double(3, v)?;
            }
            if let Some(v) = self.int_value {
                os.write_int64(4, v)?;
            }
            if let Some(v) = self.uint_value {
                os.write_uint64(5, v)?;
            }
            if let Some(v) = self.sint_value {
                os.write_sint64(6, v)?;
            }
            if let Some(v) = self.bool_value {
                os.write_bool(7, v)?;
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

        fn new() -> Value {
            Value::new()
        }

        fn clear(&mut self) {
            self.string_value = ::std::option::Option::None;
            self.float_value = ::std::option::Option::None;
            self.double_value = ::std::option::Option::None;
            self.int_value = ::std::option::Option::None;
            self.uint_value = ::std::option::Option::None;
            self.sint_value = ::std::option::Option::None;
            self.bool_value = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Value {
            static instance: Value = Value {
                string_value: ::std::option::Option::None,
                float_value: ::std::option::Option::None,
                double_value: ::std::option::Option::None,
                int_value: ::std::option::Option::None,
                uint_value: ::std::option::Option::None,
                sint_value: ::std::option::Option::None,
                bool_value: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    // @@protoc_insertion_point(message:vector_tile.Tile.Feature)
    #[derive(GodotClass)]
    #[class(init, rename=MvtFeature)]
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Feature {
        // message fields
        // @@protoc_insertion_point(field:vector_tile.Tile.Feature.id)
        pub id: ::std::option::Option<u64>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Feature.tags)
        pub tags: ::std::vec::Vec<u32>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Feature.type)
        pub type_: ::std::option::Option<::protobuf::EnumOrUnknown<GeomType>>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Feature.geometry)
        pub geometry: ::std::vec::Vec<u32>,
        // special fields
        // @@protoc_insertion_point(special_field:vector_tile.Tile.Feature.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Feature {
        fn default() -> &'a Feature {
            <Feature as ::protobuf::Message>::default_instance()
        }
    }

    #[godot_api]
    impl Feature {
        pub fn new() -> Feature {
            ::std::default::Default::default()
        }

        // optional uint64 id = 1;

        pub fn id_u64(&self) -> u64 {
            self.id.unwrap_or(0u64)
        }

        #[func]
        pub fn id(&self) -> i64 {
            self.id_u64() as i64
        }

        #[func]
        pub fn clear_id(&mut self) {
            self.id = ::std::option::Option::None;
        }

        #[func]
        pub fn has_id(&self) -> bool {
            self.id.is_some()
        }

        // Param is passed by value, moved
        pub fn set_id_u64(&mut self, v: u64) {
            self.id = ::std::option::Option::Some(v);
        }

        #[func]
        pub fn set_id(&mut self, v: i64) {
            self.set_id_u64(v as u64);
        }
        // optional .vector_tile.Tile.GeomType type = 3;

        #[func]
        pub fn geom_type(&self) -> GeomType {
            match self.type_ {
                Some(e) => e.enum_value_or(GeomType::UNKNOWN),
                None => GeomType::UNKNOWN,
            }
        }

        #[func]
        pub fn clear_type_(&mut self) {
            self.type_ = ::std::option::Option::None;
        }

        #[func]
        pub fn has_type(&self) -> bool {
            self.type_.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_type(&mut self, v: GeomType) {
            self.type_ = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
        }

        #[func]
        pub fn tags(&self) -> Array<u32> {
            Array::from_iter(self.tags.iter().map(|el| *el))
        }

        #[func]
        pub fn geometry_raw(&self) -> Array<u32> {
            Array::from_iter(self.geometry.iter().map(|el| *el))
        }

        #[func]
        pub fn geometry(&self) -> Array<Array<i32>> {
            let mut sequences = Array::new();
            let mut i = 0;
            while i < self.geometry.len() {
                let command = CommandInteger(self.geometry[i]);
                let cmd_id = command.id();
                let arg_count = command.count() as usize
                    * match cmd_id {
                        // MoveTo = 1,
                        // LineTo = 2,
                        // ClosePath = 7,
                        1 | 2 => 2,
                        _ => 0,
                    };
                i += 1;
                let seq = Array::from_iter(
                    [cmd_id as i32].into_iter().chain(
                        self.geometry[i..i + arg_count]
                            .iter()
                            .map(|arg| ParameterInteger(*arg).value()),
                    ),
                );
                i += arg_count;
                sequences.push(seq);
            }
            sequences
        }

    }

    impl ::protobuf::Message for Feature {
        const NAME: &'static str = "Feature";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.id = ::std::option::Option::Some(is.read_uint64()?);
                    },
                    18 => {
                        is.read_repeated_packed_uint32_into(&mut self.tags)?;
                    },
                    16 => {
                        self.tags.push(is.read_uint32()?);
                    },
                    24 => {
                        self.type_ = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                    },
                    34 => {
                        is.read_repeated_packed_uint32_into(&mut self.geometry)?;
                    },
                    32 => {
                        self.geometry.push(is.read_uint32()?);
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
            if let Some(v) = self.id {
                my_size += ::protobuf::rt::uint64_size(1, v);
            }
            my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.tags);
            if let Some(v) = self.type_ {
                my_size += ::protobuf::rt::int32_size(3, v.value());
            }
            my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.geometry);
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if let Some(v) = self.id {
                os.write_uint64(1, v)?;
            }
            os.write_repeated_packed_uint32(2, &self.tags)?;
            if let Some(v) = self.type_ {
                os.write_enum(3, ::protobuf::EnumOrUnknown::value(&v))?;
            }
            os.write_repeated_packed_uint32(4, &self.geometry)?;
            os.write_unknown_fields(self.special_fields.unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> Feature {
            Feature::new()
        }

        fn clear(&mut self) {
            self.id = ::std::option::Option::None;
            self.tags.clear();
            self.type_ = ::std::option::Option::None;
            self.geometry.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Feature {
            static instance: Feature = Feature {
                id: ::std::option::Option::None,
                tags: ::std::vec::Vec::new(),
                type_: ::std::option::Option::None,
                geometry: ::std::vec::Vec::new(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    // @@protoc_insertion_point(message:vector_tile.Tile.Layer)
    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(GodotClass)]
    #[class(init,rename=MvtLayer)]
    pub struct Layer {
        // message fields
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.version)
        pub version: ::std::option::Option<u32>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.name)
        pub name: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.features)
        pub features: ::std::vec::Vec<Feature>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.keys)
        pub keys: ::std::vec::Vec<::std::string::String>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.values)
        pub values: ::std::vec::Vec<Value>,
        // @@protoc_insertion_point(field:vector_tile.Tile.Layer.extent)
        pub extent: ::std::option::Option<u32>,
        // special fields
        // @@protoc_insertion_point(special_field:vector_tile.Tile.Layer.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Layer {
        fn default() -> &'a Layer {
            <Layer as ::protobuf::Message>::default_instance()
        }
    }

    #[godot_api]
    impl Layer {
        pub fn new() -> Layer {
            ::std::default::Default::default()
        }

        // required uint32 version = 15;

        #[func]
        pub fn version(&self) -> u32 {
            self.version.unwrap_or(1u32)
        }

        #[func]
        pub fn clear_version(&mut self) {
            self.version = ::std::option::Option::None;
        }

        #[func]
        pub fn has_version(&self) -> bool {
            self.version.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_version(&mut self, v: u32) {
            self.version = ::std::option::Option::Some(v);
        }

        // required string name = 1;

        pub fn name_str(&self) -> &str {
            match self.name.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        #[func]
        pub fn name(&self) -> GodotString {
            GodotString::from(self.name_str())
        }

        #[func]
        pub fn clear_name(&mut self) {
            self.name = ::std::option::Option::None;
        }

        #[func]
        pub fn has_name(&self) -> bool {
            self.name.is_some()
        }

        // Param is passed by value, moved
        pub fn set_name_string(&mut self, v: ::std::string::String) {
            self.name = ::std::option::Option::Some(v);
        }

        #[func]
        pub fn set_name(&mut self, v: GodotString) {
            self.name = Some(v.into());
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_name(&mut self) -> &mut ::std::string::String {
            if self.name.is_none() {
                self.name = ::std::option::Option::Some(::std::string::String::new());
            }
            self.name.as_mut().unwrap()
        }

        // Take field
        pub fn take_name(&mut self) -> ::std::string::String {
            self.name.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional uint32 extent = 5;

        #[func]
        pub fn extent(&self) -> u32 {
            self.extent.unwrap_or(4096u32)
        }

        #[func]
        pub fn clear_extent(&mut self) {
            self.extent = ::std::option::Option::None;
        }

        #[func]
        pub fn has_extent(&self) -> bool {
            self.extent.is_some()
        }

        // Param is passed by value, moved
        #[func]
        pub fn set_extent(&mut self, v: u32) {
            self.extent = ::std::option::Option::Some(v);
        }

        #[func]
        pub fn features(&self) -> Array<Gd<Feature>> {
            Array::from_iter(self.features.iter().map(|el| Gd::new(el.clone())))
        }

        #[func]
        pub fn keys(&self) -> Array<GodotString> {
            Array::from_iter(self.keys.iter().map(|el| el.into()))
        }

        #[func]
        pub fn values(&self) -> Array<Gd<Value>> {
            Array::from_iter(self.values.iter().map(|el| Gd::new(el.clone())))
        }
    }

    impl ::protobuf::Message for Layer {
        const NAME: &'static str = "Layer";

        fn is_initialized(&self) -> bool {
            if self.version.is_none() {
                return false;
            }
            if self.name.is_none() {
                return false;
            }
            for v in &self.features {
                if !v.is_initialized() {
                    return false;
                }
            };
            for v in &self.values {
                if !v.is_initialized() {
                    return false;
                }
            };
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    120 => {
                        self.version = ::std::option::Option::Some(is.read_uint32()?);
                    },
                    10 => {
                        self.name = ::std::option::Option::Some(is.read_string()?);
                    },
                    18 => {
                        self.features.push(is.read_message()?);
                    },
                    26 => {
                        self.keys.push(is.read_string()?);
                    },
                    34 => {
                        self.values.push(is.read_message()?);
                    },
                    40 => {
                        self.extent = ::std::option::Option::Some(is.read_uint32()?);
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
            if let Some(v) = self.version {
                my_size += ::protobuf::rt::uint32_size(15, v);
            }
            if let Some(v) = self.name.as_ref() {
                my_size += ::protobuf::rt::string_size(1, &v);
            }
            for value in &self.features {
                let len = value.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            };
            for value in &self.keys {
                my_size += ::protobuf::rt::string_size(3, &value);
            };
            for value in &self.values {
                let len = value.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            };
            if let Some(v) = self.extent {
                my_size += ::protobuf::rt::uint32_size(5, v);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if let Some(v) = self.version {
                os.write_uint32(15, v)?;
            }
            if let Some(v) = self.name.as_ref() {
                os.write_string(1, v)?;
            }
            for v in &self.features {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            };
            for v in &self.keys {
                os.write_string(3, &v)?;
            };
            for v in &self.values {
                ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
            };
            if let Some(v) = self.extent {
                os.write_uint32(5, v)?;
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

        fn new() -> Layer {
            Layer::new()
        }

        fn clear(&mut self) {
            self.version = ::std::option::Option::None;
            self.name = ::std::option::Option::None;
            self.features.clear();
            self.keys.clear();
            self.values.clear();
            self.extent = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Layer {
            static instance: Layer = Layer {
                version: ::std::option::Option::None,
                name: ::std::option::Option::None,
                features: ::std::vec::Vec::new(),
                keys: ::std::vec::Vec::new(),
                values: ::std::vec::Vec::new(),
                extent: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:vector_tile.Tile.GeomType)
    #[derive(FromGodot, ToGodot, GodotConvert)]
    pub enum GeomType {
        // @@protoc_insertion_point(enum_value:vector_tile.Tile.GeomType.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:vector_tile.Tile.GeomType.POINT)
        POINT = 1,
        // @@protoc_insertion_point(enum_value:vector_tile.Tile.GeomType.LINESTRING)
        LINESTRING = 2,
        // @@protoc_insertion_point(enum_value:vector_tile.Tile.GeomType.POLYGON)
        POLYGON = 3,
    }

    impl ::protobuf::Enum for GeomType {
        const NAME: &'static str = "GeomType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<GeomType> {
            match value {
                0 => ::std::option::Option::Some(GeomType::UNKNOWN),
                1 => ::std::option::Option::Some(GeomType::POINT),
                2 => ::std::option::Option::Some(GeomType::LINESTRING),
                3 => ::std::option::Option::Some(GeomType::POLYGON),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<GeomType> {
            match str {
                "UNKNOWN" => ::std::option::Option::Some(GeomType::UNKNOWN),
                "POINT" => ::std::option::Option::Some(GeomType::POINT),
                "LINESTRING" => ::std::option::Option::Some(GeomType::LINESTRING),
                "POLYGON" => ::std::option::Option::Some(GeomType::POLYGON),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [GeomType] = &[
            GeomType::UNKNOWN,
            GeomType::POINT,
            GeomType::LINESTRING,
            GeomType::POLYGON,
        ];
    }

    impl ::std::default::Default for GeomType {
        fn default() -> Self {
            GeomType::UNKNOWN
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_file() {
        let bytes = std::fs::read("../test/data/tile.mvt").unwrap();
        let tile = Tile::parse_from_bytes(&bytes).unwrap();
        dbg!(&tile);
        assert_eq!(tile.layers.len(), 1);
    }
}
