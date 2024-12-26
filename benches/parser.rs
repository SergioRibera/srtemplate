use divan::Bencher;
use srtemplate::SrTemplate;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_single_instance(b: Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    let ctx = SrTemplate::default();
    ctx.add_variable("variable", &"Variable");

    b.bench(|| {
        ctx.render(input).unwrap();
    })
}

#[divan::bench(args = ["This is some text. {{ variable }} and {{ toLower(trim(variable)) }}"])]
fn bench_iter_instance(input: &str) {
    let ctx = SrTemplate::default();
    ctx.add_variable("variable", &"Variable");
    ctx.render(input).unwrap();
}

#[divan::bench]
fn bench_clone_instance(b: Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    let ctx = SrTemplate::default();
    ctx.add_variable("variable", &"Variable");

    b.with_inputs(|| ctx.clone())
        .bench_values(|ctx| ctx.render(input).unwrap())
}
