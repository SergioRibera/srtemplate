use srtemplate::SrTemplate;

fn main() {
    let mut ctx = SrTemplate::default();
    ctx.add_variable("var", "World".to_string());
    ctx.add_variable("otherVar", "Other".to_string());
    
    let template = "Hello {{ var }}! This is {{ otherVar }}";
    println!("Rendered: {}", ctx.render(template).unwrap());
}
