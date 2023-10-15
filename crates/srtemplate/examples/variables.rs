use srtemplate::SrTemplate;

fn main() {
    let mut ctx = SrTemplate::default();
    ctx.add_variable("var", "World");
    ctx.add_variable("otherVar", "Other");
    ctx.add_variable("number", 85u8);
    
    let template = "Hello {{ var }}! This is {{ otherVar }} and this is number: {{number}}";
    println!("Rendered: {}", ctx.render(template).unwrap());
}
