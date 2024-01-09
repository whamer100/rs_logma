# Logma? What's Logma?
LOGMA TEXT

### todo:
- [ ] actually populate this readme (im writing this at 5 am, ill do it later)
- [ ] figure out a good way to add documentation

## Example

```rust
use logma::info;

fn main() {
  info!("logma text");
  // >>> [INFO] logma text
}
```

## Install

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
logma = "0.1.8"
colored = "2.1.0"  # this is needed for this to function, and i dont know how to get around it
```