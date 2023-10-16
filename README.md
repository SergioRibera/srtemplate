
<div align="center">

![srtemplate](https://github.com/SergioRibera/srtemplate/assets/56278796/d8e695ba-4f1b-47dd-9f70-334a4d051229)

</div>

<div align="center">

<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/srtemplate/ci.yml">
<img alt="Crates.io" src="https://img.shields.io/crates/v/srtemplate">
<img alt="docs.rs" src="https://img.shields.io/docsrs/srtemplate">

</div>

# SrTemplate
Mr. strings template is a library that allows you to render just text templates.

## Links
- You can see a real example [here](https://sergioribera.github.io/srtemplate/), it is a real time template renderer.
- Documentation [here](https://docs.rs/srtemplate/)
- [Wiki](https://github.com/SergioRibera/srtemplate/wiki)

## Features
- Super fast
- Efficient
- Renders variables of all types
- Function system
- Easy implementation of custom functions
- Minimum possible dependencies
- Feature-based implementations, use only what you need
- Using the same variables renders multiple times

## Basic example
> [!NOTE]
> See more examples [here](./examples)

```rs
use srtemplate::SrTemplate;

fn main() {
    let mut ctx = SrTemplate::default();
    ctx.add_variable("var", &"World");
    ctx.add_variable("otherVar", &"Other");
    ctx.add_variable("number", &85u8);
    
    let template = "Hello {{ var }}! This is {{ otherVar }} and this is number: {{number}}";
    println!("Rendered: {}", ctx.render(template).unwrap());
}
```

> [!NOTE]
>  For more information about the functions implemented by default or how to use the library in depth, see the [wiki](https://github.com/SergioRibera/srtemplate/wiki)

## TODO's
- [ ] I would like to change the way custom functions are implemented.
- [ ] Macros to create custom functions
- [ ] Better add_variable function to make more easy and performance managing Cow
