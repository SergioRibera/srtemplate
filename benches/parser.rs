#![feature(test)]

extern crate test;

use srtemplate::SrTemplate;
use test::Bencher;

#[bench]
fn bench_single_instance(b: &mut Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    let mut ctx = SrTemplate::default();
    ctx.add_variable("variable", "Variable".to_string());
    b.iter(|| {
        ctx.render(input).unwrap();
    })
}

#[bench]
fn bench_iter_instance(b: &mut Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    b.iter(|| {
        let mut ctx = SrTemplate::default();
        ctx.add_variable("variable", "Variable".to_string());
        ctx.render(input).unwrap();
    })
}