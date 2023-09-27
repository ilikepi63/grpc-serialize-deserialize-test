# Test GRPC between rust and javascript

## Generate javascript bindings

```
 protoc \                   
    --js_out=import_style=commonjs,binary:./js-proto \
    test.proto
```

## Generate rust bindings

This should be done automatically for you via the cargo build command. 

## Run the rust deserialization


