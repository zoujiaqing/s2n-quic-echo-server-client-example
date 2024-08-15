# echo example for s2n-quic library

## Generate cert and key

```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=localhost"
```

## Move pem(s) to certs directory

```bash
mkdir certs
mv *.pem certs/
```

## run echo server

```bash
cargo run -- server
```

## run echo client

```bash
cargo run -- client
```
