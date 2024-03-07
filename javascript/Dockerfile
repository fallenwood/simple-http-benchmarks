FROM registry.docker.com/library/node:21-bookworm-slim AS base
WORKDIR /app
RUN npm i pm2 -g

FROM base as build
COPY . .
RUN npm install

FROM base AS final
COPY --from=build /app /app
ENTRYPOINT ["pm2-runtime", "app.js", "-i", "max", "–disable-logs"]
