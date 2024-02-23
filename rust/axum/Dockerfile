FROM debian:12-slim AS base
WORKDIR /app
EXPOSE 5000

FROM registry.docker.com/library/rust:1.76.0-slim AS build
WORKDIR /src
COPY . .
RUN cargo build --release

FROM base AS final
COPY --from=build /src/target/release/axum /app/axum
ENTRYPOINT ["./axum"]