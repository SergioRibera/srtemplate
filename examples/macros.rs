use srtemplate::{function, SrTemplate, Variable};

#[function]
fn merge(name: String, age: u8) {
    Ok(format!("{name}_{age}"))
}

#[derive(Variable)]
#[template(rename = "lowercase", rename_fields = "pascal")]
pub struct User {
    name: String,
    last_name: String,
    #[template(ignore)]
    age: u8,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Sergio".into(),
            last_name: "Ribera".into(),
            age: 22,
        }
    }
}

fn main() {
    let ctx = SrTemplate::default();
    ctx.add_variable("var", &"mUnDo");
    ctx.add_variable("other", &255u8);

    ctx.add_function("merge", merge);

    ctx.add(&User::default());

    let template = "Hola {{ merge(var, other) }}, {{ user.Name }} {{ user.LastName}}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
