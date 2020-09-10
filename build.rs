extern crate protoc_rust;

use protoc_rust::Codegen;
use protoc_rust::Customize;

fn main() {
    Codegen::new()
        .out_dir("src")
        .inputs(&["protos/vector_tile.proto"])
        .include("protos")
        .run()
        .expect("protoc");
}
