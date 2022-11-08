# rust-backend
Rust backend class and proyect

## Descargar Postgress
En la carpeta data-base, ejecutar el docker-compose y listo.
```bash
$ docker-compose up -d
```

## Setup Diesel
En la carpeta raiz del proyecto **blog-platzi** ejecutar este comando (no es necesario, porque el archivo **.env** está en los fuentes).
```bash
$ echo DATABASE_URL=postgres://postgres:admin123@localhost/diesel_demo > .env
```

## Docker image

Dockerizamos el proyecto asíendo el Dockerfile con el siguiente comando lo ejecutamos. 

```bash
docker build -t web:latest . 
```