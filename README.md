This directory holds stripped down Xinu sources for the Galileo. No networking or shell code is included. This is the bare minimum to interface with the silicon.

Compile static:
`rustc --target i686-unknown-linux-gnu --crate-type=staticlib double.rs`

Use nightly: `2016-10-14`
