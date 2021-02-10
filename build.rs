fn main() {
    tonic_build::compile_protos("proto/v1.proto").unwrap();
    tonic_build::compile_protos("proto/v1alpha2.proto").unwrap();
}
