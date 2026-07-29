use std::{env, path::PathBuf};

fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-changed=proto/blog.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = out_dir.join("blog_descriptor.bin");

    let mut config = prost_build::Config::new();
    config
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types");

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        // .extern_path(".google.protobuf", "::pbjson_types")
        // .compile_protos(&["proto/blog.proto"], &["proto"])?;
        .compile_with_config(config, &["proto/blog.proto"], &["proto"])?;

    let descriptor_set = std::fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)
        .unwrap()
        //     .exclude(prefixes)
        .build(&[".blog"])
        .unwrap();

    Ok(())
}
