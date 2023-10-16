use srtemplate::prelude::{to_typed_args, validations, FuncResult};
use srtemplate::SrTemplate;

fn merge(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?; // We validate that we receive a minimum of 1 argument.
    let _just_one = to_typed_args::<(String,)>(args)?;
    let _raw_args = to_typed_args::<(String, u8)>(args)?;
    let raw_args: (String, u8) = to_typed_args(args)?;

    println!("Args: {raw_args:?}");

    let (a, b) = raw_args;
    Ok(format!("{a}_{b}"))
}

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");
    ctx.add_variable("other", &255u8);

    ctx.add_function("merge", merge);

    let template = "Hola {{ merge(var, other) }}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
