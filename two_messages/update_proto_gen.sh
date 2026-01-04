#!/bin/bash

# Update protobuf generated files using Docker
docker run -it \
    --user "$(id -u):$(id -g)" \
    -v "$(pwd):/home/developer/sources" \
    protoc_builder:0.1.0 \
    sh -c "cd sources/proto_src && protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust_out=../proto_gen *.proto"
