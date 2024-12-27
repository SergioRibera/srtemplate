use srtemplate::SrTemplate;

fn main() {
    let ctx = SrTemplate::default();

    let template = "Hi {{ toTitle(var) }}";

    match ctx.render(template) {
        Ok(result) => println!("Rendered: {result}"),
        Err(e) => match e {
            srtemplate::Error::BadSyntax(e) => println!("Invalid syntaxis: {e}"),
            srtemplate::Error::VariableNotFound(e) => println!("Variable not found: {e}"),
            srtemplate::Error::FunctionNotImplemented(e) => {
                println!("Function not supported: {e}")
            }
            srtemplate::Error::Function(e) => println!("Error procesing function: {e}"),
        },
    }
}
