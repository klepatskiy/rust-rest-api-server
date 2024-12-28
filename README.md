# Description
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

use sqlx migrate to create a migration
```shell
  sqlx migrate add <migration_name>
```
