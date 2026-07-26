fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-changed=proto/blog.proto");
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/blog.proto"], &["proto"])?;

    Ok(())
}
