FROM registry.docker.com/library/eclipse-temurin:21-jre AS base
WORKDIR /app

FROM registry.docker.com/library/eclipse-temurin:21-jdk AS build
WORKDIR /src
COPY . .
RUN ./gradlew build

FROM base AS final
COPY --from=build /src/build/libs/ktor-sample-all.jar ./
ENTRYPOINT ["java","-jar","./ktor-sample-all.jar"]
