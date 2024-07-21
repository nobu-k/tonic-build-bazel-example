use std::io::Result;

fn main() -> Result<()> {
    #[cfg(feature = "bazel")]
    {
        let r = runfiles::Runfiles::create()?;
        let protoc_loc = std::env::var("PROTOC_RLOCATION").expect("PROTOC_RLOCATION must be set");
        let protoc_path = r.rlocation(protoc_loc.as_str());
        let protoc_dir = protoc_path.parent().unwrap();
        std::env::set_var(
            "PATH",
            format!(
                "{}:{}",
                protoc_dir.to_str().unwrap(),
                std::env::var("PATH").unwrap(),
            ),
        );
    }
    // TODO: add file_descriptor_set_path
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&["./hello.proto"], &["."])?;
    Ok(())
}
