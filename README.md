# pandog (🐼🐶)

Wrapped [pandoc](https://pandoc.org), universal document converter using Rust.


## Getting Started

* Clone the repository
```bash
git clone git@github.com:kawaeee/pandog.git
cd pandog/
```

* Run `pandog` development server
```bash
cargo run
```

* Build `pandog` binary using cargo
```bash
cargo build --release
```

* Serve Docker container as [local API endpoint](http://localhost:8080/convert)
```bash
# Directly build and run
docker build -t pandog-api-image .
docker run --rm --name=pandog-api-container -p 0.0.0.0:8080:7878 pandog-api-image

# Serve with docker compose
docker-compose build
docker-compose up
```
