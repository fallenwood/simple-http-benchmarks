FROM mcr.microsoft.com/dotnet/runtime:8.0  AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY dotnet.csproj /src/dotnet.csproj
COPY Program.cs /src/Program.cs
RUN dotnet publish -c Release -o out

FROM base AS final
COPY --from=build /src/out /app/dotnet
COPY appsettings.json /app/appsettings.json

ENV DOTNET_gcServer=1
ENV ASPNETCORE_URLS=http://+:5000
ENV ASPNETCORE_ENVIRONMENT=Production

ENTRYPOINT ["dotnet", "./dotnet.dll"]