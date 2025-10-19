import Keycloak from "keycloak-js";
import React, {useEffect, useState} from "react";
import {AuthContext} from "./AuthContext.tsx";

const _keycloak = new Keycloak({
    url: "http://localhost:8989",
    realm: "myrealm",
    clientId: "my-public-client",
});

export const AuthContextKeycloakProvider = ({ children }: { children: React.ReactNode }) => {

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

    _keycloak.onAuthSuccess = () => {
        if (_keycloak.tokenParsed?.preferred_username && _keycloak.tokenParsed?.preferred_username !== user) {
            setUser(_keycloak.tokenParsed?.preferred_username);
        }
    }

    if (!_keycloak.didInitialize) {
        _keycloak
            .init({
                onLoad: "check-sso",
                silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
            })
            .catch((err) => {
                console.error("Keycloak init error:", err);
            })
    }

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
        setUser(undefined);
        void _keycloak.logout();
    }

    const username = () => {
        return user
    }

    const hasRole = (roles: string[]) => {

        alert(`roles: ${_keycloak.tokenParsed}`)

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