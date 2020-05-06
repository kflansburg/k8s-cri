fn main() {
    tonic_build::compile_protos("proto/cri/v1alpha2/api.proto").unwrap();
}
