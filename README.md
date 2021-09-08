# Watson, a tool for searching social accounts, insipred by sherlock-py

## Usage
```rust
use watson::*;

fn main() {
    let watson = WatsonBuilder::new("i3ima").load_json(None).build();
    watson.check_hosts(&watson.hosts);
}
```

## TODO

- [X] Load hosts list at compile-time and store it as static string
- [X] Adapt sherlock-py json model
- [X] Write some tests
- [X] Rework junky init-code
- [X] Handle all possible scenarious of response
- [X] Parallelization with rayon
- [ ] logging and reports generation 
- [X] Documentation
- [ ] Add error handling. But should I?
- [ ] Add dockerfile
- [ ] Interactive search after full completion