# chaos-droid
repo for the stuff where we run chaos on odroids

# Building gRPC

Gotta install the right stuff but then you

```
protoc --rust_out=src/protos --grpc_out=src/protos --plugin=protoc-gen-grpc=C:/Users/andrewcsmith/.cargo/bin/grpc_rust_plugin.exe src/protos/chaos.proto
```
