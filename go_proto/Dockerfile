FROM golang:1.24-alpine3.22 AS builder
WORKDIR /app
COPY go_proto .
COPY ./proto/greeter.proto /proto/greeter.proto
RUN apk add protoc --no-cache && \
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest && \
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest && \
    go generate && \
    go build

FROM alpine:3.22 AS runner
WORKDIR /app
COPY --from=builder /app/go_proto .
ENTRYPOINT [ "./go_proto" ]
