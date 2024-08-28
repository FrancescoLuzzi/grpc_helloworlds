fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "dockerproto")]
    tonic_build::compile_protos("/app/greeter.proto")?;
    #[cfg(not(feature = "dockerproto"))]
    tonic_build::compile_protos("../proto/greeter.proto")?;
    Ok(())
}
