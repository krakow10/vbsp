# VBSP

Rust parser for valve bsp files.

Currently only supports the tf2 version of bsp files, but adding other sourcemod variants should be fairly straight forward.

# Example usage

```rust
extern crate bsp;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let data = std::fs::read("maps/cp_steel.bsp")?;
    let bsp = bsp::Bsp::read(&data)?;
    println!("{:?}", bsp);

    Ok(())
}
```

## Credits

This project is adapter from the [quake bsp parser] and
wouldn't be possible without information from the [source engine wiki].

[quake bsp parser]: (https://github.com/Vurich/bsp)
[source engine wiki]: (https://developer.valvesoftware.com/wiki/Source_BSP_File_Format)