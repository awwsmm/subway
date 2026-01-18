# subway

This app consists of four main components
- a database (Postgres)
- a backend (written in Rust)
- a frontend (written in TypeScript)
- an auth service (Keycloak)

## `backend/`

Contains the Rust / Salvo / Diesel backend.

To run the backend by itself, see [backend/README.md](backend/README.md).

## `frontend/`

Contains the TypeScript / React / Vite frontend.

To run the frontend by itself, see [frontend/README.md](frontend/README.md).

## `keycloak/`

Contains [Keycloak](https://www.keycloak.org/) configuration (for user authentication and authorization).

## fullstack development

### Containerized

(Note that building the application in this way (with Docker) requires an Internet connection.)

To run the full-stack application, you must first build the `frontend`, `backend`, and `keycloak` Docker container images.

```shell
docker build -t subway-frontend -f frontend/Dockerfile .
```

You must create a TLS certificate and key for the backend (which requires HTTPS)

<!-- TODO fix this so we don't need the "-k" flag -->

```shell
mkdir -p backend/certs && openssl req -x509 -newkey rsa:4096 -keyout backend/certs/key.pem -out backend/certs/cert.pem -sha256 -days 47 -nodes -subj '/CN=localhost'
```

...and then create the backend image

```shell
docker build -t subway-backend -f backend/Dockerfile .
```

Similarly, you must also create a TLS certificate and key for Keycloak (which requires HTTPS)

```shell
mkdir -p keycloak/certs && openssl req -x509 -newkey rsa:4096 -keyout keycloak/certs/key.pem -out keycloak/certs/cert.pem -sha256 -days 47 -nodes -subj '/CN=localhost'
```

...and then create the keycloak image

```shell
docker build -t subway-keycloak -f keycloak/Dockerfile .
```

Finally, you can run the full stack with `docker-compose`

```shell
docker-compose up
```

and visit http://localhost:5173 in the browser

Note that records are persisted on your local disk when the application is shut down. If you want to clear the database, run

```shell
docker-compose down -v --remove-orphans
```

The above two processes have been encoded in scripts for ease of use. Simply run

```shell
./up
```

to bring the whole application stack up, and

```shell
./down
```

to bring it all down again.

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

Note that when the backend restarts, the database will be wiped; similarly, when the frontend restarts (or you close the tab / window), you will need to re-authenticate as your dummy user of choice.

## `keycloak`

`subway` uses [keycloak](https://www.keycloak.org/) for user authentication and authorization.

`keycloak/realm-export.json` configures some default users for testing and development.

With the app running, visit http://localhost:5173/login to view the login page, or http://localhost:5173/user-only to view a protected page.

LoginPage with the dummy users `admin` (password `admin`), `bob` (password `bob`), or `clara` (password `clara`).

### protected endpoints

There is a protected backend endpoint at https://localhost:7878/user-only

If you try to access it unauthorized...

```shell
curl -k https://localhost:7878/user-only
```

...you will get a response like

```
Missing or malformed x-token header
```

You must first acquire a token via the `/login` endpoint to proceed

```shell
export TOKEN=$(curl -k -X POST -H "Content-Type: application/json" -d '{"username":"bob","password":"bob"}' https://localhost:7878/login)
```

You can then `curl` this endpoint like

```shell
curl -k -H "x-token: $TOKEN" https://localhost:7878/user-only
```

You should receive a response like

```
welcome, bob!
```

Similarly, there is an `admin-only` endpoint, which can only be accessed by the `admin` user

```shell
export TOKEN=$(curl -k -X POST -H "Content-Type: application/json" -d '{"username":"admin","password":"admin"}' https://localhost:7878/login)
```

You can then `curl` this endpoint like

```shell
curl -k -H "x-token: $TOKEN" https://localhost:7878/admin-only
```

You should receive a response like

```
welcome, administrator
```

To login in to the backend directly via Keycloak, as the frontend does, first, get an auth token and an id token from Keycloak

```shell
export KC_UNAME="clara"; export KC_PWD=$KC_UNAME; \
 eval $(curl -k -X POST https://localhost/realms/myrealm/protocol/openid-connect/token \
  -d "client_id=my-confidential-client" \
  -d "client_secret=my-client-secret" \
  -d "grant_type=password" \
  -d "username=$KC_UNAME" \
  -d "password=$KC_PWD" \
  -d "scope=openid" | jq -r '"export ATOKEN=\(.access_token) ITOKEN=\(.id_token)"')
```

Then, use the `/login-keycloak` endpoint

```shell
export TOKEN=$(curl -k -H "x-keycloak-access-token: $ATOKEN" -H "x-keycloak-id-token: $ITOKEN" -H "x-keycloak-realm: myrealm" https://localhost:7878/login)
```

Finally, use the other endpoints as normal

```shell
curl -k -H "x-token: $TOKEN" https://localhost:7878/user-only
```

You should receive a response like

```
welcome, clara!
```

The `/login` endpoint is required for backend auth when in-memory authentication is used, but (as can be seen above) it's also a convenient shortcut when Keycloak is in use. When Keycloak is being used, both the `/login` and `/login-keycloak` endpoints authenticate via Keycloak, but the `/login` one gets the auth and id tokens and parses them automatically.