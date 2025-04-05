use std::fmt::Display;

use srtemplate::{function, SrTemplate, Variable};

#[function]
fn merge(name: String, age: u8) {
    Ok(format!("{name}_{age}"))
}

#[derive(Variable)]
#[template(case_fields = "pascal")]
pub struct User {
    name: String,
    last_name: String,
    #[template(ignore)]
    age: u8,
    #[template(rename = "correo")]
    email: String,
}

#[derive(Variable)]
#[template(rename = "testunit")]
pub struct TestUnit;

impl Display for TestUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestUnit")
    }
}

#[derive(Variable)]
pub struct TestTuple(u8, u8);

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Sergio".into(),
            last_name: "Ribera".into(),
            email: "sergioribera@rustlang-es.org".into(),
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
    ctx.add(&TestUnit);
    ctx.add(&TestTuple(5, 12));

    let template = "Hola {{ merge(var, other) }}, {{ User.Name }} {{ User.LastName}} @ {{ User.Correo }}. {{ testunit }} {{ TestTuple.0 }} {{ TestTuple.1 }}";

    println!("Rendered: {}", ctx.render(template).unwrap());
}
