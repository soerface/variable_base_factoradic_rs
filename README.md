# Variable-base Factoradic numbers

Implementation of https://xkcd.com/2835/

## Installation

https://crates.io/crates/variable_base_factoradic

```bash
cargo install variable_base_factoradic
```

## Usage

```rust
use variable_base_factoradic::VariableBaseFactoradicNumber;

// Convert from decimal to factoradic
let v = VariableBaseFactoradicNumber::try_new(5038).unwrap();
println!("{}", v.to_string()); // 654320

// Convert from factoradic to decimal
let v = "654320".parse::<VariableBaseFactoradicNumber>().unwrap();
println!("{}", v.value); // 5038
```