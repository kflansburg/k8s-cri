fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);
    let proto = std::path::Path::new("proto/cri/v1alpha2/api.proto");
    let proto_dir = proto
        .parent()
        .expect("proto file should reside in a directory");
    tonic_build::configure()
        .compile_with_config(config, &[proto], &[proto_dir])
        .unwrap();
}
