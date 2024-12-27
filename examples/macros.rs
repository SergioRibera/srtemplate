use srtemplate::{function, SrTemplate};

#[function]
fn merge(name: String, age: u8) {
    Ok(format!("{name}_{age}"))
}

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");
    ctx.add_variable("other", &255u8);

    ctx.add_function("merge", merge);

    let template = "Hola {{ merge(var, other) }}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
