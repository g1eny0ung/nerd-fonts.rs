# nerd-fonts.rs

[![Build Status](https://travis-ci.org/g1eny0ung/nerd-fonts.rs.svg?branch=master)](https://travis-ci.org/g1eny0ung/nerd-fonts.rs)

[Nerd Fonts](https://github.com/ryanoasis/nerd-fonts) in rust.

## Preinstall

[https://github.com/ryanoasis/nerd-fonts#font-installation](https://github.com/ryanoasis/nerd-fonts#font-installation)

## How to use

In your `Cargo.toml`, add:

```toml
[dependencies]
nerd_fonts = "0.1"
```

Then import it:

```rust
extern crate nerd_fonts;
```

Use `NerdFonts::load` to load:

```rust
use nerd_fonts::NerdFonts;

let nf = NerdFonts {
    nf: NerdFonts::load(),
};

let nf_custom_c = nf.get("custom-c").unwrap(); // '\u{e61e}'
```

## How to contribute

Pull a request or open an issue to describe your changes or problems.

## License

MIT@g1eny0ung
