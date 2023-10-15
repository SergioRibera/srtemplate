# SrTemplate
Mr. strings template is a library that allows you to render just text templates.

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
> [!INFO] **NOTE:** See more examples [here](./crates/srtemplate/examples)

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

> [!INFO] For more information about the functions implemented by default or how to use the library in depth, see the [wiki](https://github.com/SergioRibera/srtemplate/wiki)

## TODO's
- [ ] I would like to change the way custom functions are implemented.
- [ ] Macros to create custom functions
