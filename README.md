# TBD


```shell
# get okp4d and install it. requires go installed
git clone https://github.com/okp4/okp4d
cd okp4d
make build
make install
```

# Instructions

Run local node
```
just wasm
```

Run local node
```
make okp4-local
```

Deploy smart contract
```
RUST_LOG=info cargo run --example test-local-okp4
```

