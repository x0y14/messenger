fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/operation_service.proto")?;
    tonic_build::compile_protos("../proto/supervisor_service.proto")?;
    tonic_build::compile_protos("../proto/types.proto")?;
    Ok(())
}