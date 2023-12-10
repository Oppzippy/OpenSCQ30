use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["protobuf/device_state.proto"], &["protobuf/"])?;
    Ok(())
}
