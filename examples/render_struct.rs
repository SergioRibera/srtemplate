use srtemplate::SrTemplate;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("(x: {}, y: {})", self.x, self.y)
    }
}

fn main() {
    let ctx = SrTemplate::default();

    let p = Point { x: 0.0, y: 5.025 };

    ctx.add_variable("point", &p);

    let template = "Point {{ point }}";
    println!("Rendered: {}", ctx.render(template).unwrap());
}
