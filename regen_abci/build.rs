extern crate protoc_rust_grpc;
extern crate protoc_rust;

use protoc_rust::Customize;


fn main() {
//    protoc_rust_grpc::run(protoc_rust_grpc::Args {
//        out_dir: "src",
//        includes: &[],
//        input: &["src/x/sig/codec.proto"],
//        rust_protobuf: true,
//        ..Default::default()
//    }).expect("protoc-rust-grpc");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/x/sig/",
        input: &["src/x/sig/codec.proto"],
        includes: &[],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");}