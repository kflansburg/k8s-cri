fn main() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/v1.proto"], &["proto/"])
        .unwrap();

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/v1alpha2.proto"], &["proto/"])
        .unwrap();
}
