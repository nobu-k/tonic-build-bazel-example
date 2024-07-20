use std::io::Result;

fn main() -> Result<()> {
    // TODO: add file_descriptor_set_path
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["../protos/hello.proto"], &["../protos"])?;
    Ok(())
}
