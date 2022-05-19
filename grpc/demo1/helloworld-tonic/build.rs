fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::configure()
//          .build_server(false)
//          .compile(
//              &["proto/helloworld/helloworld.proto"],
//              &["proto/helloworld"],
//          )?;
//     Ok(())
//  }