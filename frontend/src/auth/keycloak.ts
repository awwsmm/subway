import Keycloak from "keycloak-js";

const keycloak = new Keycloak({
    url: "http://localhost:8989",
    realm: "myrealm",
    clientId: "my-public-client",
});

export default keycloak;
