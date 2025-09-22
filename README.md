# subway

## `backend/`

Contains the Rust / Salvo / Diesel backend.

To run the backend by itself, see [backend/README.md](backend/README.md).

## `frontend/`

Contains the TypeScript / React / Vite frontend.

To run the frontend by itself, see [frontend/README.md](frontend/README.md).

## fullstack development

To run the full-stack application, you must first build both the `frontend` and the `backend` Docker container images.

Note that building the application in this way (with Docker) requires an Internet connection.

```shell
cd frontend && docker build -t subway-frontend . && cd ..
```

```shell
cd backend && docker build -t subway-backend . && cd ..
```

then, you can run the full stack with `docker-compose`

```shell
docker-compose up
```

Note that records are persisted on your local disk when the application is shut down. If you want to clear the database, run

```shell
docker-compose down -v --remove-orphans
```

## `keycloak`

`subway` uses [keycloak](https://www.keycloak.org/) for user authentication and authorization.

`keycloak/realm-export.json` configures some default users for testing and development.