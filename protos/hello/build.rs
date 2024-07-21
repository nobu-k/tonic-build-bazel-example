use std::io::Result;

fn main() -> Result<()> {
    // TODO: add file_descriptor_set_path
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&["./hello.proto"], &["."])?;
    Ok(())
}
