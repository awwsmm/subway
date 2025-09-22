import Keycloak from "keycloak-js";

const keycloak = new Keycloak({
    url: "http://localhost:8989",       // Keycloak base URL (adjust if needed)
    realm: "myrealm",                   // Replace with your realm
    clientId: "my-react-client",        // Replace with your client ID
});

export default keycloak;
