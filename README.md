# memfrob-rs

A rust implementation of the [memfrob](https://man7.org/linux/man-pages/man3/memfrob.3.html)
function from GNU C Library.

## Usage example

```rust
use memfrob::memfrob;

fn main() {
	let mut buffer = [42; 15];
	memfrob(&mut buffer);
	// buffer is now frobnicated.
}
```

Please note that this function is not useable for encryption - see the note on the manpage.
