Some filters that ignore input values and return constant values

# Examples

```rust
use literal_funs::*;

let foo = (0..4).map(bool::<true>).collect::<Vec<_>>();
assert_eq!(foo, [true; 4]);

let foo = (0..4).map(bool::<false>).collect::<Vec<_>>();
assert_eq!(foo, [false; 4]);

let foo = (0..4).map(i32::<16>).collect::<Vec<_>>();
assert_eq!(foo, [16; 4]);
```
