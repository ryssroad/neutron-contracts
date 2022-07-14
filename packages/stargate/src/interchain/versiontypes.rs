// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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

//! Generated file from `tendermint/version/versiontypes.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  App includes the protocol and software version for the application.
///  This information is included in ResponseInfo. The App.Protocol can be
///  updated in ResponseEndBlock.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.version.App)
pub struct App {
    // message fields
    // @@protoc_insertion_point(field:tendermint.version.App.protocol)
    pub protocol: u64,
    // @@protoc_insertion_point(field:tendermint.version.App.software)
    pub software: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.version.App.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a App {
    fn default() -> &'a App {
        <App as ::protobuf::Message>::default_instance()
    }
}

impl App {
    pub fn new() -> App {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "protocol",
            |m: &App| { &m.protocol },
            |m: &mut App| { &mut m.protocol },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "software",
            |m: &App| { &m.software },
            |m: &mut App| { &mut m.software },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<App>(
            "App",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for App {
    const NAME: &'static str = "App";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.protocol = is.read_uint64()?;
                },
                18 => {
                    self.software = is.read_string()?;
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
        if self.protocol != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.protocol);
        }
        if !self.software.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.software);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.protocol != 0 {
            os.write_uint64(1, self.protocol)?;
        }
        if !self.software.is_empty() {
            os.write_string(2, &self.software)?;
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

    fn new() -> App {
        App::new()
    }

    fn clear(&mut self) {
        self.protocol = 0;
        self.software.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static App {
        static instance: App = App {
            protocol: 0,
            software: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for App {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("App").unwrap()).clone()
    }
}

impl ::std::fmt::Display for App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for App {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Consensus captures the consensus rules for processing a block in the blockchain,
///  including all blockchain data structures and the rules of the application's
///  state transition machine.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tendermint.version.Consensus)
pub struct Consensus {
    // message fields
    // @@protoc_insertion_point(field:tendermint.version.Consensus.block)
    pub block: u64,
    // @@protoc_insertion_point(field:tendermint.version.Consensus.app)
    pub app: u64,
    // special fields
    // @@protoc_insertion_point(special_field:tendermint.version.Consensus.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Consensus {
    fn default() -> &'a Consensus {
        <Consensus as ::protobuf::Message>::default_instance()
    }
}

impl Consensus {
    pub fn new() -> Consensus {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "block",
            |m: &Consensus| { &m.block },
            |m: &mut Consensus| { &mut m.block },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "app",
            |m: &Consensus| { &m.app },
            |m: &mut Consensus| { &mut m.app },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Consensus>(
            "Consensus",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Consensus {
    const NAME: &'static str = "Consensus";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.block = is.read_uint64()?;
                },
                16 => {
                    self.app = is.read_uint64()?;
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
        if self.block != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.block);
        }
        if self.app != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.app);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.block != 0 {
            os.write_uint64(1, self.block)?;
        }
        if self.app != 0 {
            os.write_uint64(2, self.app)?;
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

    fn new() -> Consensus {
        Consensus::new()
    }

    fn clear(&mut self) {
        self.block = 0;
        self.app = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Consensus {
        static instance: Consensus = Consensus {
            block: 0,
            app: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Consensus {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Consensus").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Consensus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Consensus {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tendermint/version/versiontypes.proto\x12\x12tendermint.version\x1a\
    \x14gogoproto/gogo.proto\"=\n\x03App\x12\x1a\n\x08protocol\x18\x01\x20\
    \x01(\x04R\x08protocol\x12\x1a\n\x08software\x18\x02\x20\x01(\tR\x08soft\
    ware\"9\n\tConsensus\x12\x14\n\x05block\x18\x01\x20\x01(\x04R\x05block\
    \x12\x10\n\x03app\x18\x02\x20\x01(\x04R\x03app:\x04\xe8\xa0\x1f\x01B;Z9g\
    ithub.com/tendermint/tendermint/proto/tendermint/versionJ\x8b\x06\n\x06\
    \x12\x04\0\0\x17\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x01\0\x1b\n\x08\n\x01\x08\x12\x03\x03\0P\n\t\n\x02\x08\x0b\x12\x03\
    \x03\0P\n\t\n\x02\x03\0\x12\x03\x05\0\x1e\n\xb7\x01\n\x02\x04\0\x12\x04\
    \n\0\r\x01\x1a\xaa\x01\x20App\x20includes\x20the\x20protocol\x20and\x20s\
    oftware\x20version\x20for\x20the\x20application.\n\x20This\x20informatio\
    n\x20is\x20included\x20in\x20ResponseInfo.\x20The\x20App.Protocol\x20can\
    \x20be\n\x20updated\x20in\x20ResponseEndBlock.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\n\x08\x0b\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x02\x16\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x04\x0b\x02\n\r\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\
    \x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\t\x11\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x0b\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x02\
    \x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0c\x02\x0b\x16\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x0c\t\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x14\x15\n\xc7\x01\n\
    \x02\x04\x01\x12\x04\x12\0\x17\x01\x1a\xba\x01\x20Consensus\x20captures\
    \x20the\x20consensus\x20rules\x20for\x20processing\x20a\x20block\x20in\
    \x20the\x20blockchain,\n\x20including\x20all\x20blockchain\x20data\x20st\
    ructures\x20and\x20the\x20rules\x20of\x20the\x20application's\n\x20state\
    \x20transition\x20machine.\n\n\n\n\x03\x04\x01\x01\x12\x03\x12\x08\x11\n\
    \n\n\x03\x04\x01\x07\x12\x03\x13\x02\"\n\r\n\x06\x04\x01\x07\x8d\xf4\x03\
    \x12\x03\x13\x02\"\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x15\x02\x13\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x15\x02\x13\"\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x15\t\x0e\
    \n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x15\x11\x12\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x03\x16\x02\x13\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x16\
    \x02\x15\x13\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x16\x02\x08\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x16\t\x0c\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x16\x11\x12b\x06proto3\
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
            deps.push(super::gogo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(App::generated_message_descriptor_data());
            messages.push(Consensus::generated_message_descriptor_data());
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
