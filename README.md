# Doyle, a tool for searching social-media accounts, insipred by sherlock-py

![crates.io](https://img.shields.io/crates/v/doyle.svg)

## Usage
```rust
use doyle::*;

fn main() {
    let doyle = DoyleBuilder::new("i3ima").load_json(None).build();
    doyle.check_hosts(&doyle.hosts);
}
```

<details open>
<summary>TODO</summary>

- [X] Load hosts list at compile-time and store it as static string
- [X] Adapt sherlock-py json model
- [X] Write some tests
- [X] Rework junky init-code
- [X] Handle all possible scenarious of response
- [X] Parallelization with rayon
- [ ] logging and reports generation 
- [X] Documentation
- [ ] Add error handling. But should I?
- [ ] Interactive search after full completion

</details>