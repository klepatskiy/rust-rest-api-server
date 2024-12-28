# server grpc
rust rest api server axum


## Setup
Докер контрейнеры с postgres и openapi-generator
```shell
  docker compose up -d
```

Выполнить миграции
```shell
  make migrate
```


Generate request and response DTO from open api 
```shell
  make speca
```

use sqlx migrate to create a migration
```shell
  sqlx migrate add <migration_name>
```# rust-rest-api-server
