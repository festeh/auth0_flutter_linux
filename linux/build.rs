use lib_flutter_rust_bridge_codegen::codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = codegen::Config {
        rust_root: Some(".".into()),
        rust_input: Some("crate::api".into()),
        dart_output: Some(".".into()),
        ..Default::default()
    };
    let meta = codegen::MetaConfig { watch: false };

    codegen::generate(config, meta)?;
    Ok(())
}
