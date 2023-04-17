FROM rust:1.68.2 as builder

WORKDIR /app

COPY . .

# Build rust binary
RUN cargo build --release

FROM ubuntu:jammy as deployer

WORKDIR /app

ENV LANG=C.UTF-8
ENV DEBIAN_FRONTEND=noninteractive

# Update apt package
RUN apt update --fix-missing

# Install required dependencies
RUN apt install -y pandoc pandoc-citeproc texlive

# Remove cache
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
RUN apt autoclean
RUN apt autoremove

COPY --from=builder /app/target/release/pandog /app/pandog

CMD ["/app/pandog"]