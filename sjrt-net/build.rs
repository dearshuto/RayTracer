use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = Path::new("resources/sjrt.proto");
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .out_dir("./src/detail/generated")
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
