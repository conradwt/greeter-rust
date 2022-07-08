# Greeter Rust

## Running Server

```zsh
cargo run --bin greeter-server
```

## Testing Server

```zsh
grpcurl -plaintext -import-path ./proto -proto greeter.proto -d '{"name": "Conrad"}' localhost:50051 greeterapi.Greeter.SayHello
```

## Running Client

```zsh
cargo run --bin greeter-client
```

## References

- TBD

## License

Greeter Rust is released under the [MIT license](./LICENSE.md).

## Copyright

copyright:: (c) Copyright 2022 Conrad Taylor. All Rights Reserved.
