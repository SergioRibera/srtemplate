use srtemplate::SrTemplate;

fn main() {
    let ctx = SrTemplate::default();

    let template = "Hi {{ toTitle(var) }}";

    match ctx.render(template) {
        Ok(result) => println!("Rendered: {result}"),
        Err(e) => match e {
            srtemplate::SrTemplateError::BadSyntax(e) => println!("Invalid syntaxis: {e}"),
            srtemplate::SrTemplateError::VariableNotFound(e) => println!("Variable not found: {e}"),
            srtemplate::SrTemplateError::FunctionNotImplemented(e) => {
                println!("Function not supported: {e}")
            }
            srtemplate::SrTemplateError::Function(e) => println!("Error procesing function: {e}"),
        },
    }
}
