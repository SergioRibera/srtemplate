use srtemplate::prelude::FunctionError;
use srtemplate_macros::custom_function;

fn validate_string(s: String) -> Result<(), FunctionError> {
    Ok(())
}

#[custom_function]
pub fn gen_name(_hash: String, _n: u32, a: f32) -> Result<String, FunctionError> {
    // Código de la función principal
    Ok("random_name".to_string())
}

fn main() {
}
