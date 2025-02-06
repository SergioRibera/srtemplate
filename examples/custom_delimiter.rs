use srtemplate::SrTemplate;

fn main() {
    let mut ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");

    let template = "Hola {{ toLower(var) }}";

    println!("Rendered: {}", ctx.render(template).unwrap());

    // Change delimiter
    ctx.set_delimiter("||", "||");
    let template = "Hola || toLower(var) ||";

    println!("Rendered: {}", ctx.render(template).unwrap());

    // Creating new instance of template like shell syntax
    let ctx = SrTemplate::with_delimiter("${", "}");
    ctx.add_variable("var", &"mUnDo");
    let template = "Hola ${ toLower(var)}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
