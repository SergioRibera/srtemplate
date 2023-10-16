use srtemplate::prelude::{validations, FuncResult};
use srtemplate::SrTemplate;

fn to_title(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?; // We validate that we receive a minimum of 1 argument.

    println!("Args: {args:?}");
    Ok(args
        .iter()
        .map(|a| {
            let first = a.get(0..1).unwrap_or_default().to_uppercase();
            let last = a.get(1..).unwrap_or_default().to_lowercase();
            format!("{first}{last}")
        })
        .collect::<Vec<String>>()
        .join(" "))
}

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");
    ctx.add_function("toTitle", to_title);

    let template = "Hola {{ toTitle(var) }}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
