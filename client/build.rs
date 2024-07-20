use std::io::Result;

fn main() -> Result<()> {
    // TODO: add file_descriptor_set_path
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["../protos/hello.proto"], &["../protos"])?;
    Ok(())
}
