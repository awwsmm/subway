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

    const [initialized, setInitialized] = useState(() => {
        const initialized = sessionStorage.getItem('subway-initialized');
        return initialized ? JSON.parse(initialized) : false;
    });

    // Use useEffect to save state to sessionStorage whenever 'initialized' changes
    useEffect(() => {
        sessionStorage.setItem('subway-initialized', JSON.stringify(initialized));
    }, [initialized]); // Dependency array: effect runs when 'initialized' changes

    // init() is called during login()
    const init = () => {
        if (initialized) {
            // do nothing

        } else {
            _keycloak
                .init({
                    onLoad: "check-sso",
                    silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
                })
                .then(() => {
                    setInitialized(true);
                })
                .catch((err) => {
                    console.error("Keycloak init error:", err);
                })
        }
    }

    const loggedIn = () => {
        return !!user;
    }

    const login = () => {
        if (loggedIn()) {
            // do nothing

        } else {
            init();

            void _keycloak.login();
            setUser(_keycloak.tokenParsed?.preferred_username || "<unknown>");

            // const username = prompt(`Enter username to login (e.g., ${users.join(", ")}):`);
            // if (username && users.includes(username)) {
            //     setUser(username);
            //
            // } else {
            //     alert('Invalid user. Staying logged out.');
            // }
        }
    }

    const logout = (redirectUri: string) => {
        void _keycloak.logout();
        setUser(undefined);
        window.location.href = redirectUri;
    }

    const username = () => {
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