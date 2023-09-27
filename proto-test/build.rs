fn main() {
    tonic_build::configure()
        .out_dir("src/")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["test.proto"], &["../"])
        .unwrap();
}
