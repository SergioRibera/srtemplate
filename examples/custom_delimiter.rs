use srtemplate::SrTemplate;

fn main() {
    let mut ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");

    let template = "Hola {{ toLower(var) }}";

    println!("Rendered: {}", ctx.render(template).unwrap());

    // Change delimiter
    // need mut instance
    ctx.set_delimiter("||", "||");
    let template = "Hola || toLower(var) ||";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
