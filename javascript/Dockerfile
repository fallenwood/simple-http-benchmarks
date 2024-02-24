FROM registry.docker.com/library/node:21-bookworm-slim AS base
WORKDIR /app

FROM base as build
COPY . .
RUN npm install

FROM base AS final
COPY --from=build /app /app
ENTRYPOINT ["node", "app.js"]
