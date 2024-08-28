# proto_test

## Setup

```shell
apt install -y protobuf-compiler
```

## Testing

```shell
# install cli tool
go install github.com/vadimi/grpc-client-cli/cmd/grpc-client-cli@latest

# usage
grpc-client-cli --proto ./proto/greeter.proto localhost:50052
```

## Go proto

```shell
docker build . -f go_proto/Dockerfile -t proto:go
```

## Rust proto

```shell
docker build . -f rust_proto/Dockerfile -t proto:rust
```
