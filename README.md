# subway

## `backend/`

Contains the Rust / Salvo / Diesel backend.

To run the backend by itself, see [backend/README.md](backend/README.md).

## `frontend/`

Contains the TypeScript / React / Vite frontend.

To run the frontend by itself, see [frontend/README.md](frontend/README.md).

## fullstack development

### Containerized

To run the full-stack application, you must first build both the `frontend` and the `backend` Docker container images.

Note that building the application in this way (with Docker) requires an Internet connection.

```shell
cd frontend && docker build -t subway-frontend . ; cd ..
```

```shell
cd backend && docker build -t subway-backend . ; cd ..
```

then, you can run the full stack with `docker-compose`

```shell
docker-compose up
```

and visit http://localhost:5173 in the browser

Note that records are persisted on your local disk when the application is shut down. If you want to clear the database, run

```shell
docker-compose down -v --remove-orphans
```

### Hot Reloading

If Docker is unnecessary for the development work you're doing (if you don't need a persistent database or realistic authentication, etc.), you can develop in _hot reloading_ mode. This is usually the easiest way to develop.

Run the backend from the `backend/` directory in one terminal with the command

```shell
bacon run-long
```

...and run the frontend from the `frontend/` directory in a different terminal with

```shell
npm run dev
```

Then visit http://localhost:5173 in the browser.

In _hot reloading_ mode, you never need to re-build or re-run either the frontend or the backend. Simply save your changes and `bacon` / `npm` will automatically restart the backend / frontend, respectively.

Note that when the backend restarts, the database will be wiped; similarly, when the frontend restarts, you will need to re-authenticate as your dummy user of choice.

## `keycloak`

`subway` uses [keycloak](https://www.keycloak.org/) for user authentication and authorization.

`keycloak/realm-export.json` configures some default users for testing and development.

With the app running, visit http://localhost:5173/login to view the login page, or http://localhost:5173/protected to view a protected page.

Login with the dummy users `admin` (password `admin`), `bob` (password `bob`), or `clara` (password `clara`).