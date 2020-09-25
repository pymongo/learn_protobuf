fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(protobuf_codegen_pure::Customize::default())
        // .customize(protobuf_codegen_pure::Customize {..Default::default()})
        .out_dir("src/protos")
        .input("src/protos/user.proto")
        .include("src/protos")
        .run()
        .unwrap();
}
