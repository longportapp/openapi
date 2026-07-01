# openapi-protobufs
All Protobuf files for OpenAPI

## Development

After modifying any Protobuf files, you must run `buf generate` to regenerate the code for all supported languages (Go, Python, C++).

```bash
buf generate
```

This will generate code based on the configuration in `buf.gen.yaml` and output to the `gen/` directory.
