# dir-size

`dir-size` is a crate that calculates directory size in parallel using `rayon`.

## Usage

This is a little code sample:

```rust
use dir_size::{get_size_in_bytes, get_size_in_human_bytes};
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let path = Path::new("/home");
    println!("{} bytes", get_size_in_bytes(path)?);
    println!("{}", get_size_in_human_bytes(path)?);
    Ok(())
}
```