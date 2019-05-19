## Example of macro error

To reproduce, run

```bash
wasm-pack build
```

Few more details:
```bash
# this works fine
cargo build

# this produces the same error
cargo build --target wasm32-unknown-unknown
```
