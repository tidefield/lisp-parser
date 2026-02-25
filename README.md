# lisp-parser

A recursive descent parser for Lisp to build an AST

### Prerequisite

https://doc.rust-lang.org/cargo/getting-started/installation.html

### Running and testing

```
$ cargo run '(first (list 1 (+ 2 3) 9))'
["first", ["list", 1, ["+", 2, 3], 9]]
$ cargo test
```

### Unhandled corner cases

* Invalidating leftover tokens. `(+ 1 2) garbage` parses as `(+ 1 2)` without error.
* Parsing negative numbers
* Erroring on empty input
* Supporting more arithmetic operators such as `-`, `*`, `/`
