fn main() -> Result<(), Box<dyn std::error::Error>> {
   let proto_file = "message.proto";

   tonic_build::configure()
           .build_server(true)
           .build_client(false)
           .compile(&[proto_file], &["../proto/"])?;

   Ok(())
}