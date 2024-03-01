FROM debian:12-slim AS base
WORKDIR /app
EXPOSE 8080

FROM registry.docker.com/library/rust:1.76.0-slim AS prep
WORKDIR /src
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources
RUN apt update -y && apt install -y cmake libfindbin-libs-perl

FROM prep AS build
COPY . .
ENV HTTP_PROXY=http://192.168.1.102:10801
ENV HTTPS_PROXY=http://192.168.1.102:10801
ENV ALL_PROXY=http://192.168.1.102:10801
RUN cargo build --release

FROM base AS final
COPY --from=build /src/target/release/pingora /app/pingora
COPY ./config.yaml /app/config.yaml
ENTRYPOINT ["./pingora"]
