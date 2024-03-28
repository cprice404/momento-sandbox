fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/hello.proto")?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated_protos")
        .compile(&["proto/hello.proto"], &["proto"])?;

    Ok(())
}
