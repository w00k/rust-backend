# rust-backend
Rust backend class and proyect

## Descargar Postgress
```bash
$ docker run --name diesel_demo -e POSTGRES_PASSWORD=admin123 -d postgres
```

## Setup Diesel
```bash
echo DATABASE_URL=postgres://postgres:admin123@localhost/diesel_demo > .env
```