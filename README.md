# PQC certificate verifier

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

pqc-verify-wasm validates self-signed trust anchor files containing certificates signed using quantum resistant algorithms, such as those found in the [pqc-certificates](https://github.com/IETF-Hackathon/pqc-certificates) repo from the IETF PQC Certificates Hackathon. 
Certificate validation support is provided by the certval library in the [rust-pki](https://github.com/carl-wallace/rust-pki) repo. 
DER decoding support is provided via crates from the RustCrypto [formats](https://github.com/RustCrypto/formats) repo. 
At present, only Dilithium certificates are supported for wasm32-unknown-unknown target (Sphincsplus and Falcon are supported by certval for other targets).

Testing has been performed using [trunk](https://trunkrs.dev). A build is temporarily available [here](https://downloads.redhoundsoftware.com/temp/dist/index.html). 

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of development.

## Minimum Supported Rust Version

This crate requires **Rust 1.65** at a minimum.

The MSRV may change in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
[RFC 5937]: https://datatracker.ietf.org/doc/html/rfc5937
