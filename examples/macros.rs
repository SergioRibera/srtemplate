use srtemplate::{function, SrTemplate, Variable};

#[function]
fn merge(name: String, age: u8) {
    Ok(format!("{name}_{age}"))
}

#[derive(Default, Variable)]
#[template(rename = "lowercase", rename_fields = "pascal")]
pub struct User {
    name: String,
    last_name: String,
    #[template(ignore)]
    age: u8,
}

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");
    ctx.add_variable("other", &255u8);

    ctx.add_function("merge", merge);

    ctx.add(&User::default());

    let template = "Hola {{ merge(var, other) }}, {{ user.name }} {{ user.age }}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
