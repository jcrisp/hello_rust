## Simple Times Table Rust Service

### Setup

#### Local

```
cargo install && target/release/hellorust
```

#### Docker

```
docker build -t jcrisp/hello_rust hello_rust
docker run -p 8080:8080 jcrisp/hello_rust
```

### Hit the service

http://localhost:8080/?table=3
