use std::io::Error;
use prost_build::Config;

fn main() -> Result<(), Error>{
    Config::new()
        .out_dir("src/schema")
        .compile_protos(&["src/schema/api.v1.proto"], &["src/schema"])?;
    Ok(())
}