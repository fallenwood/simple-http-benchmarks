FROM registry.docker.com/library/eclipse-temurin:21-jre AS base
WORKDIR /app

FROM registry.docker.com/library/eclipse-temurin:21-jdk AS build
WORKDIR /src
COPY . .
RUN ./gradlew build

FROM base AS final
COPY --from=build /src/build/libs/demo-0.0.1-SNAPSHOT.jar ./
ENTRYPOINT ["java","-jar","./demo-0.0.1-SNAPSHOT.jar"]
