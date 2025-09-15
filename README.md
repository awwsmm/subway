# subway

## `backend/`

Contains the Rust / Salvo / Diesel backend.

To run the backend by itself, see [backend/README.md](backend/README.md).

## `frontend/`

Contains the TypeScript / React frontend.

To run the frontend by itself, see [frontend/README.md](frontend/README.md).

## to run

To run the full-stack application, you must first build both the `frontend` and the `backend` Docker container images

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