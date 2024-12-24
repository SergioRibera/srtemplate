use srtemplate::SrTemplate;

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"World");
    ctx.add_variable("otherVar", &"Other");
    ctx.add_variable("number", &85u8);

    let template = String::from(
        "Hello {{ var }}! This is {{ otherVar }} and this is number: {{ add_u8(number, 10) }}",
    );
    println!("Rendered: {}", ctx.render(template).unwrap());
}
