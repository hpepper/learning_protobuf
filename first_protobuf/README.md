# Learning protobuf for rust

- [Protocol Buffers Documentation - Rust Reference](https://protobuf.dev/reference/rust/)
- [Language Guide - editions](https://protobuf.dev/programming-guides/editions/)
- [rust files](https://protobuf.dev/reference/rust/rust-generated/)
- [](https://users.rust-lang.org/t/protocol-buffers-in-rust-what-to-use-and-how-to-use-it/88647)
- [](https://protobuf.dev/support/version-support/)
- [](https://github.com/protocolbuffers/protobuf/releases)

- The edition must be the first non-empty, non-comment line of the file: [Language Guide - editions](https://protobuf.dev/programming-guides/editions/).
- You should use the field numbers 1 through 15 for the most-frequently-set fields. Lower field number values take less space in the wire format.
  - field numbers in the range 1 through 15 take one byte to encode. Field numbers in the range 16 through 2047 take two bytes. 
  - [Protocol Buffer Encoding](https://protobuf.dev/programming-guides/encoding/#structure)
- It’s recommended to include as few message types per .proto file as possible, [](https://protobuf.dev/programming-guides/editions/).

- proto_src - proto source files.
- proto_gen - generated rust code from the .proto files.

You need to add 'protobuf = "4.33.2-release"' it must fit the version used for protoc
