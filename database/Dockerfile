FROM rustlang/rust:nightly-bookworm-slim AS rust

FROM rust as builder

COPY . .
RUN apt-get update && apt-get install -y clang python3 python3-pip
RUN pip install cmake --break-system-packages

RUN python3 ./x.py build -- database
