# CitiesAPI

## Database migrations

## Run migrations

```sh
DATABASE_URL=postgres://postgres:password@localhost:5432/city_api sqlx migrate run
```

## Development

### Development compose

In the included compose file is included a Postgres 18 instance.

### Pre-commit

```sh
uv tool install pre-commit
pre-commit install
```
