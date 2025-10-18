import Keycloak from "keycloak-js";
import React, {useEffect, useState} from "react";
import {AuthContext} from "./AuthContext.tsx";

const _keycloak = new Keycloak({
    url: "http://localhost:8989",
    realm: "myrealm",
    clientId: "my-public-client",
});

export const AuthContextKeycloakProvider = ({ children }: { children: React.ReactNode }) => {

    // alert("AuthContextKeycloakProvider constructed")

    /*


{state: "9b40d7af-08c2-45fe-8ffb-69e8bf127662", nonce: "4985be54-edca-44a3-9a7b-6cf1799781ae",…}
expires
:
1760792698270
nonce
:
"4985be54-edca-44a3-9a7b-6cf1799781ae"
pkceCodeVerifier
:
"gqPkk4GkzIFhNOcDLBqhrNO57Nvi5gkoVUekzHDT2wJoqJ4ePswhyOc8qa8WUxxEMGO8Z0AZCjqe0MSimpOadGgnlGP8RHil"
redirectUri
:
"http%3A%2F%2Flocalhost%3A5173%2Flogin"
state
:
"9b40d7af-08c2-45fe-8ffb-69e8bf127662"

     */

    const [user, setUser] = useState(() => {
        const maybeUser = sessionStorage.getItem('subway-user');
        return maybeUser || undefined;
    });

    // Use useEffect to save state to sessionStorage whenever 'user' changes
    useEffect(() => {
        if (user) {
            sessionStorage.setItem('subway-user', user);
        } else {
            sessionStorage.removeItem('subway-user');
        }
    }, [user]); // Dependency array: effect runs when 'user' changes


    // TODO works as expected!
    _keycloak.onAuthSuccess = () => {
        // alert(`keycloak auth success with user ${_keycloak.tokenParsed?.preferred_username}`)
        if (_keycloak.tokenParsed?.preferred_username && _keycloak.tokenParsed?.preferred_username !== user) {
            setUser(_keycloak.tokenParsed?.preferred_username);
        }
    }

    // TODO works as expected!
    _keycloak.onReady = () => {
        // alert(`keycloak ready with user ${_keycloak.tokenParsed?.preferred_username}`)
        if (_keycloak.tokenParsed?.preferred_username && _keycloak.tokenParsed?.preferred_username !== user) {
            setUser(_keycloak.tokenParsed?.preferred_username);
        }
    }


    //
    // const [initialized, setInitialized] = useState(() => {
    //     const initialized = sessionStorage.getItem('subway-initialized');
    //     return initialized ? JSON.parse(initialized) : false;
    // });
    //
    // // Use useEffect to save state to sessionStorage whenever 'initialized' changes
    // useEffect(() => {
    //     sessionStorage.setItem('subway-initialized', JSON.stringify(initialized));
    // }, [initialized]); // Dependency array: effect runs when 'initialized' changes

    if (!_keycloak.didInitialize) {
        _keycloak
            .init({
                onLoad: "check-sso",
                silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
            })
            .then(() => {
                // TODO works as expected!
                // alert(`Keycloak initialized with user ${_keycloak.tokenParsed?.preferred_username}`);
                if (_keycloak.tokenParsed?.preferred_username && _keycloak.tokenParsed?.preferred_username !== user) {
                    setUser(_keycloak.tokenParsed?.preferred_username);
                }
                // setInitialized(true);
                // setUser(_keycloak.tokenParsed?.preferred_username);
            })
            .catch((err) => {
                console.error("Keycloak init error:", err);
            })
    }

    // alert(`username == ${_keycloak.tokenParsed?.preferred_username}, user = ${user}`)

    // if (_keycloak.tokenParsed?.preferred_username !== user || (!!_keycloak.tokenParsed?.preferred_username && !!user) ) {
    //     setUser(_keycloak.tokenParsed?.preferred_username || undefined);
    // }

    // init() is called during login()
    const init = () => {
        return;
    }

    const loggedIn = () => {
        return !!username();
    }

    const login = () => {
        if (loggedIn()) {
            // do nothing

        } else {
            init();

            _keycloak.login()
                .then(() => setUser(_keycloak.tokenParsed?.preferred_username));
            // setUser(_keycloak.tokenParsed?.preferred_username || "<unknown>");

        }
    }

    const logout = (_redirectUri: string) => {
        setUser(undefined)
        // setInitialized(false)
        _keycloak.logout()
            // .then(() => setUser(undefined))
            // .then(() => setInitialized(false))
            // .then(() => window.location.href = redirectUri)
        // setUser(undefined);
        // setInitialized(false);
        // window.location.href = redirectUri;
    }

    const username = () => {
        // return _keycloak.tokenParsed?.preferred_username || undefined;
        return user
    }

    const hasRole = (roles: string[]) => {

        const userRoles = _keycloak.tokenParsed?.realm_access?.roles || [];
        return (userRoles && userRoles.some(role => roles.includes(role))) || false;

        // const userRoles = realmExport.users.find(user => user.username === username())?.realmRoles
        // return (userRoles && userRoles.some(role => roles.includes(role))) || false;
    }

    return (
        <AuthContext value={{ loggedIn, login, logout, username, hasRole }}>
            {children}
        </AuthContext>
    );
};