import Keycloak from "keycloak-js";
import React, {useEffect, useState} from "react";
import {AuthContext} from "./AuthContext.tsx";

const _keycloak = new Keycloak({
    url: "http://localhost:8989",
    realm: "myrealm",
    clientId: "my-public-client",
});

export const AuthContextKeycloakProvider = ({ children }: { children: React.ReactNode }) => {

    // Keycloak stores its tokens in memory, which is wiped on a page refresh (the Keycloak object is reconstructed).
    // In order to keep username, roles, etc. persistent across refreshes, we save them to session storage.
    // Session storage is persistent as long as the tab / window is not closed.
    // TODO expire this information based on the token expiry date retrieved on Keycloak initialization
    //   (save token expiry time in session storage, as well) (consider security implications of this (XSS))

    // On a tab or window close-and-reopen, Keycloak still sees the user as logged in, but session storage is wiped.
    // To handle that case, we do a "check-sso" initialization (see below) which does a silent login.
    // If the user did not log in, closes the tab, and opens a new one, "check-sso" allows unauthenticated access, as
    // well -- it does not redirect the user to a login page.

    // default 'user' value is pulled from session storage
    const [_username, set_username] = useState(sessionStorage.getItem('subway_username') || undefined);

    // whenever setUser() is called, update 'user' in session storage
    useEffect(() => {
        if (_username) {
            sessionStorage.setItem('subway_username', _username);
        } else {
            sessionStorage.removeItem('subway_username');
        }
    }, [_username]); // run this effect when any of these values change
    
    // default 'roles' value is pulled from session storage
    const [_roles, set_roles] = useState(() => {
        const sessionRoles = sessionStorage.getItem('subway_roles');
        return sessionRoles ? JSON.parse(sessionRoles) as string[] : [];
    });

    // whenever setRoles() is called, update 'roles' in session storage
    useEffect(() => {
        sessionStorage.setItem('subway_roles', JSON.stringify(_roles));
    }, [_roles]); // run this effect when any of these values change

    // hook into Keycloak and set the 'user' and 'roles' after successful authentication
    // any information which is needed for rendering should be included in this component's state
    _keycloak.onAuthSuccess = () => {
        if (_keycloak.tokenParsed?.preferred_username && _keycloak.tokenParsed?.preferred_username !== _username) {
            set_username(_keycloak.tokenParsed?.preferred_username || undefined);
            set_roles(_keycloak.tokenParsed?.realm_access?.roles || []);
        }
    }

    // if Keycloak is not yet initialized, initialize it
    if (!_keycloak.didInitialize) {
        _keycloak
            // if there is a 'user' in session storage, require (silent) Keycloak authentication ("login-required")
            // otherwise, allow the user to use the app unauthenticated ("check-sso")
            .init({ onLoad: _username ? "login-required" : "check-sso" })
            .catch((err) => console.error("Keycloak init error:", err))
    }

    // init() is a no-op in this implementation of AuthContext
    const init = () => {
        return;
    }

    // the user is logged in if we know their username
    const loggedIn = () => {
        return !!username();
    }

    // delegate login functionality to Keycloak
    const login = () => {
        if (loggedIn()) {
            // do nothing

        } else {
            init(); // call this no-op so linter doesn't mark it as unused
            void _keycloak.login();
        }
    }

    // clean up session storage on logout
    const logout = (_redirectUri: string) => {
        set_username(undefined);
        set_roles([]);
        void _keycloak.logout({ logoutMethod: "POST" }); // "POST" means the user does not go to a "logout" page
    }

    // pull this info from session storage, not directly from Keycloak
    const username = () => {
        return _username;
    }

    // pull this info from session storage, not directly from Keycloak
    const hasRole = (roles: string[]) => {
        return _roles.some(role => roles.includes(role));
    }

    return (
        <AuthContext value={{ loggedIn, login, logout, username, hasRole }}>
            {children}
        </AuthContext>
    );
};