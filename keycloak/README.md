# keycloak

This directory contains a file, `realm-export.json`, which configures the Keycloak Docker container for local development and integration testing. As JSON does not support comments, this README serves as the line-by-line explanation of that file.

---

```json
{
  "realm": "myrealm",
```

Keycloak provides a `"master"` realm by default, but we create our own realm called `"myrealm"`. This is where all the users, clients, etc., that we create will live. It is bad practice to do so in the `"master"` realm

> This realm was created for you when you first started Keycloak. It contains the administrator account you created at the first login. Use the master realm only to create and manage the realms in your system. [[ source ]](https://www.keycloak.org/docs/latest/server_admin/index.html#the-master-realm)

---

```json
  "sslRequired": none,
```

This means we can access Keycloak over `http://`, and not just `https://`.

> TODO make sure SSL is required when deploying Keycloak in production.

---

```json
  "enabled": true,
```

The realm is enabled.

---

```json
  "clients": [
    {
      "clientId": "my-confidential-client",
```

The main Keycloak concepts we care about in our `realm-export.json` file are
- clients
- roles, and
- users

The code above declares our first client, called `my-confidential-client`.

To clarify "users" vs. "clients" -- "users" are the actual humans using your app. The app itself is the "client". The user provides their login information to the client, which sends a request to Keycloak to verify that login information. This project has two clients, the backend and the frontend, each with their own Keycloak `client`.

---

```json
      "enabled": true,
```

`my-confidential-client` is enabled.

---

```json
      "protocol": "openid-connect",
```

There are [a few protocols](https://www.keycloak.org/docs/latest/server_admin/#sso-protocols) which we can use, like OpenID Connect (aka. OIDC) and SAML, but Keycloak recommend OIDC.

> For most purposes, Keycloak recommends using OIDC. [[ source ]](https://www.keycloak.org/docs/latest/server_admin/index.html#ref-saml-vs-oidc_server_administration_guide)

---

```json
        "publicClient": false,
```

`my-confidential-client` is a "confidential" (as opposed to "public") client. (There used to be a "bearer-only" client type, as well, but this has been deprecated.) We use a confidential client because it provides an extra layer of protection: the client must also provide a "client secret". Essentially, both the client and the user must provide their own username and password

> Confidential clients provide client secrets when they exchange the temporary codes for tokens. Public clients are not required to provide client secrets. Public clients are secure when HTTPS is strictly enforced and redirect URIs registered for the client are strictly controlled. [[ source ]](https://www.keycloak.org/docs/latest/server_admin/index.html#con-oidc-auth-flows_server_administration_guide)

However

> HTML5/JavaScript clients have to be public clients because there is no way to securely transmit the client secret to HTML5/JavaScript clients. [[ source ]](https://www.keycloak.org/docs/latest/server_admin/index.html#con-oidc-auth-flows_server_administration_guide)

> There is definitely no good way to handle secrets on the client side... the client is not under your control and can never be trusted. [[ source ]](https://community.auth0.com/t/storing-client-secret-in-spa/22717/2)

...so we also have a public `my-public-client` for the frontend, which will be explained later.

---

```json
      "secret": "my-client-secret",
```

The client secret is essentially the client's password. It lets Keycloak know that this is a known, authorized client. It should be protected like a password.

> TODO in production, treat confidential client secrets like passwords. Do not store them in plaintext.

---

```json
      "redirectUris": [
        "http://localhost:5173/*"
      ],
```

This field defines all valid URIs to which the user can be redirected after authorization. It exists to prevent redirection to a third party after authorization, where the authenticated user could then be impersonated.

> TODO this will have to be changed for production, as well.

---

```json
      "defaultClientScopes": [
        "profile", "roles"
      ],
```

"Default client scopes" are the information which is included in access tokens provided to clients by default (without specifically asking for it in the request).

> A default scope that is attached to a client using the OpenID Connect protocol will automatically use the protocol mappers defined within that scope to build claims for this client regardless of the provided OAuth2.0 `scope` parameter. [[ source ]](https://registry.terraform.io/providers/edflex-tech/keycloak/latest/docs/resources/openid_client_default_scopes)

The "roles" scope is included here so that user roles ("user", "admin", etc.) are made available in the token, so that content and functionality can be shown / hidden from users in the frontend (e.g. only admins should be able to create new users).

Keycloak also has a concept of "groups" -- how do groups differ from roles?

> In Keycloak, Groups are just a collection of users that you can apply roles and attributes to in one place. Roles define a type of user and applications assign permission and access control to roles [[ source ]](https://wjw465150.gitbooks.io/keycloak-documentation/content/server_admin/topics/groups/groups-vs-roles.html)

However, fine-grained permissions [should be handled in the application](https://stackoverflow.com/q/66354281/2925434), not in Keycloak.

---

```json
      "optionalClientScopes": [
        "email"
      ],
```

"Optional client scopes" are the information which _may_ be included in access tokens, _if_ the client requests it.

> An optional scope that is attached to a client using the OpenID Connect protocol will allow a client to request it using the OAuth 2.0 `scope` parameter. When requested, the scope's protocol mappers defined within that scope will be used to build claims for this client.

Any scopes not included in either `defaultClientScopes` or `optionalClientScopes` are totally inaccessible to the client. For example, the `"phone"` and `"address"` scopes are not included in either the default or the optional lists here, and so that information cannot be retrieved by this client.

---

```json
      "clientAuthenticatorType": "client-secret",
```

`clientAuthenticatorType` defines the mechanism by which a client authenticates itself to Keycloak. Options include `"client-secret"`, `"client-jwt"`, `"client-x509"`, and possibly others (documentation is not great).

Since we are authenticating with a `"secret"`, we use `"client-secret"` here.

When we create a public client, we omit this field as well as `"secret"` and change `"publicClient"` to `true`.

---

```json
      "directAccessGrantsEnabled": true,
```

This is required to avoid errors like...

```json
{"error":"unauthorized_client","error_description":"Client not allowed for direct access grants"}
```

...when making `curl` requests like

```shell
curl -X POST http://localhost:8989/realms/myrealm/protocol/openid-connect/token \
  -d "client_id=my-confidential-client" \
  -d "client_secret=my-client-secret" \
  -d "grant_type=password" \
  -d "username=bob" \
  -d "password=bob" \
  -d "scope=openid"
```

when using a confidential client.

> TODO: this is not prod ready, because per "...Best Current Practice for OAuth 2.0 Security (RFC 9700), this flow MUST NOT be used, preferring alternative methods such as Device Authorization Grant or Authorization code." [[ source ]](https://www.keycloak.org/securing-apps/oidc-layers#_resource_owner_password_credentials_flow)

In the frontend application, the [Authorization Code Flow](https://www.keycloak.org/securing-apps/oidc-layers#_authorization_code) should be used instead.

Note: `-d "scope=openid"` above is required to get the `"sub"` information in the _ID token_ (_not_ the _access token_). `"sub"` refers to the "subject" of the token--in this case, `"bob"`--and holds the subject's user ID (a UUID).

The above `curl` request returns a response like

```json
{"access_token":"eyJ...4IQ","expires_in":300,"refresh_expires_in":1800,"refresh_token":"eyJ...GTA","token_type":"Bearer","id_token":"eyJ...eCg","not-before-policy":0,"session_state":"55d3cfbb-0594-4a69-b853-83d15c04eec8","scope":"openid profile"}
```

The `access_token` and `id_token` can be decoded using a tool like https://www.jwt.io/ to inspect their contents. The decoded `access_token` will look something like

```json
{
  "exp": 1760906329,
  "iat": 1760906029,
  "jti": "onrtro:61b2bdd0-d403-9f89-5e99-cea205794395",
  "iss": "http://localhost:8989/realms/myrealm",
  "typ": "Bearer",
  "azp": "my-confidential-client",
  "sid": "652809cd-9f35-492e-b358-f040bf4dd3b1",
  "realm_access": {
    "roles": [
      "user"
    ]
  },
  "scope": "openid profile",
  "name": "Bob User",
  "preferred_username": "bob",
  "given_name": "Bob",
  "family_name": "User"
}
```

while the decoded `id_token` will look something like

```json
{
  "exp": 1760359940,
  "iat": 1760359640,
  "jti": "a0019df9-7370-0e74-c316-a613a6fc9783",
  "iss": "http://localhost:8989/realms/myrealm",
  "aud": "my-confidential-client",
  "sub": "7f16300f-6063-41ef-9428-ced32ef5adad",
  "typ": "ID",
  "azp": "my-confidential-client",
  "sid": "7e675b31-9841-44e5-b4a4-44ae42bca6ca",
  "at_hash": "uHUl9PtVRuABezMMlDfjLQ",
  "name": "Bob User",
  "preferred_username": "bob",
  "given_name": "Bob",
  "family_name": "User"
}
```

The ID token answers the question: "who is the user?" (what's their unique identifying information?), while the access token answers the question: "what can the user do?" (what roles do they have?).

---

```json
      "defaultRoles": []
```

Finally, the last line of `"clients"` is this line, which says that users of this client will be given no permissions by default. They must be assigned roles explicitly.

---

```json
    {
      "clientId": "my-public-client",
      "enabled": true,
      "publicClient": true,
      "redirectUris": [
        "http://localhost:5173/*"
      ],
      "defaultClientScopes": [
        "profile", "roles"
      ],
      "optionalClientScopes": [
        "email"
      ],
      "defaultRoles": []
    }
```

Here we add another client, a public one, which is used by the frontend. Because it is impossible to securely deliver a client secret to the browser, any frontend which communicates with Keycloak must be a public client. 

---

```json
  "roles": {
```

Here we begin the section where we define roles for the realm and the clients in the realm.

---

```json
    "realm": [
      {
        "id": "role-id-1",
        "name": "admin",
        "description": "realm-export.json-defined admin role"
      },
      {
        "id": "role-id-2",
        "name": "user",
        "description": "realm-export.json-defined user role"
      }
    ],
```

We define realm-scoped roles, which can apply to clients and to users, regardless of client.

We _could_ have another section (`"client"`) here for client-specific roles, and populate it with roles specific to that client, like...

```json
    "client": {
      "my-confidential-client": [
        {
          "name": "client-admin",
          "description": "client-admin role"
        },
        {
          "name": "client-user",
          "description": "client-user role"
        }
      ]
    }
```

...but this is not currently necessary. If we ever need client-scoped roles, we can add them in this way.

---

```json
  "users": {
```

Here we begin the section where we define users for the realm and the clients in the realm.

---

```json
    {
      "username": "admin",
      "enabled": true,
      "emailVerified": true,
      "firstName": "Admin",
      "lastName": "User",
      "email": "admin@user.com",
      "realmRoles": [
        "admin", "user"
      ],
      "credentials": [
        {
          "type": "password",
          "value": "admin",
          "temporary": false
        }
      ]
    }
```

The first predefined user is `"admin"`, which has the `"admin"` and `"user"` roles. The `"admin"` user uses `"password"` verification, with the password `"admin"`.

> TODO obviously, this is not prod-ready. Do not blindly use `realm-export.json` when deploying this app to a production environment.

---

```json
    {
      "username": "bob",
      "enabled": true,
      "emailVerified": true,
      "firstName": "Bob",
      "lastName": "User",
      "email": "bob@user.com",
      "realmRoles": [
        "user"
      ],
      "credentials": [
        {
          "type": "password",
          "value": "bob",
          "temporary": false
        }
      ]
    }
```

Other users are defined to test other app functionality. The users `"bob"` and `"clara"` both have only the `"user"` role.

To distinguish between the `"admin"` user and `"bob"` / `"clara"`, we can use the `"realmRoles"` in the `access_token` (see above) -- `"bob"` and `"clara"` will not have the `"admin"` role.

To distinguish between `"bob"` and `"clara"`, we can use the `"sub"` field of the `id_token` (see above) -- this is a unique ID string (a UUID) which is generated by Keycloak for each user. Once created, `sub` cannot be changed, and so can be used to uniquely identify a user in perpetuity.
