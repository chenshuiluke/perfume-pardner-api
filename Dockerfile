FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src

# Copy csproj and restore as distinct layers
COPY ["perfume-pardner-api.csproj", "./"]
COPY ["src/PerfumePardner.Core/PerfumePardner.Core.csproj", "src/PerfumePardner.Core/"]
COPY ["src/PerfumePardner.Infrastructure/PerfumePardner.Infrastructure.csproj", "src/PerfumePardner.Infrastructure/"]
RUN dotnet restore "./perfume-pardner-api.csproj"

# Copy everything else and build
COPY . .
RUN dotnet publish -c Release -o /app

# Build runtime image
FROM mcr.microsoft.com/dotnet/aspnet:8.0
WORKDIR /app
COPY --from=build /app .
ENTRYPOINT ["dotnet", "perfume-pardner-api.dll"]