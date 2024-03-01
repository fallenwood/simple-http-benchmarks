FROM debian:12-slim AS base
WORKDIR /app
EXPOSE 5000

FROM registry.docker.com/library/rust:1.76.0-slim AS prep
WORKDIR /src
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources
RUN apt update -y && apt install -y cmake libfindbin-libs-perl

FROM prep AS build
COPY . .
RUN cargo build --release

FROM base AS final
COPY --from=build /src/target/release/pingora /app/pingora
ENTRYPOINT ["./pingora"]
