#![feature(test)]

extern crate test;

use srtemplate::SrTemplate;
use test::Bencher;

#[bench]
fn bench_single_instance(b: &mut Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    let ctx = SrTemplate::default();
    ctx.add_variable("variable", &"Variable");

    b.iter(|| {
        ctx.render(input).unwrap();
    })
}

#[bench]
fn bench_iter_instance(b: &mut Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    b.iter(|| {
        let ctx = SrTemplate::default();
        ctx.add_variable("variable", &"Variable");
        ctx.render(input).unwrap();
    })
}

#[bench]
fn bench_clone_instance(b: &mut Bencher) {
    let input = "This is some text. {{ variable }} and {{ toLower(trim(variable)) }}";

    let ctx = SrTemplate::default();
    ctx.add_variable("variable", &"Variable");

    b.iter(|| {
        for _i in 0..100 {
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                ctx.render(input).unwrap();
            });
        }
    })
}
