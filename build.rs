fn main() {
    tonic_prost_build::configure()
        .out_dir("src/") // specify the generated code's location
        .compile_protos(&["proto/zkp_auth.proto"], &["proto/"])
        .unwrap();
}
