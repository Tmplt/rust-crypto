# Rust-Crypto

This fork is a project in conjunction with [eAESy-RTFM](https://github.com/Tmplt/eAESy-RTFM).
This code-base will be stripped to only support the AES algorithm to then utilize the dedicated AES hardware offered on the micro controllers used in eAESy-RTFM.

## Project status

- [x] Remove any files not related to AES or not depended upon by AES
- [ ] Remove everything not AES
- [ ] Rewrite non-rust components into Rust (esp. asm)
- [ ] Rewrite code-base to support `#![no_std]` (if necessary)
- [ ] ???
- [ ] Merge into eAESy-RTFM

## License

Rust-Crypto is dual licensed under the MIT and Apache 2.0 licenses, the same licenses
as the Rust compiler.
