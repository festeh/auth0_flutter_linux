use lib_flutter_rust_bridge_codegen::codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = codegen::Config {
        rust_root: Some("./".into()),
        rust_input: Some("crate::api".into()),
        dart_output: Some(".".into()),
        ..Default::default()
    };
    codegen::generate(config, Default::default())?;
    Ok(())
}
