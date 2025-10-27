import React, {useEffect, useState} from "react";
import realmExport from "../../../keycloak/realm-export.json";
import {AuthContext} from "./AuthContext.tsx";

export const AuthContextInMemoryProvider = ({ children }: { children: React.ReactNode }) => {

    // This in-memory AuthContext implementation saves its info in session storage / external files (realm-export.json),
    // to persist across refreshes.
    // If the tab / window is closed and the app is reopened in another tab / window, that info will be lost.
    // For an implementation which persists across tab / window close-and-reopens, see AuthContextKeycloak.

    // default 'username' value is pulled from session storage
    const [_username, set_username] = useState(sessionStorage.getItem('subway_username') || undefined);

    // whenever set_username() is called, update 'username' in session storage
    useEffect(() => {
        if (_username) {
            sessionStorage.setItem('subway_username', _username);
        } else {
            sessionStorage.removeItem('subway_username');
        }
    }, [_username]); // run this effect when any of these values change

    // default 'initialized' value is pulled from session storage
    const [_initialized, set_initialized] = useState(() => {
        const initialized = sessionStorage.getItem('subway_initialized');
        return initialized ? JSON.parse(initialized) : false;
    });

    // whenever set_initialized() is called, update 'initialized' in session storage
    useEffect(() => {
        sessionStorage.setItem('subway_initialized', JSON.stringify(_initialized));
    }, [_initialized]); // run this effect when any of these values change

    // init() is called during login()
    const init = () => {
        if (_initialized) {
            // do nothing

        } else {
            if (realmExport) {
                set_initialized(true);
            }
        }
    }

    const login = () => {
        if (loggedIn()) {
            // do nothing

        } else {
            init();

            const users = realmExport.users.map(each => each.username);
            const username = prompt(`Enter username to login (e.g., ${users.join(", ")}):`);

            if (username && users.includes(username)) {
                set_username(username);

            } else {
                alert('Invalid user. Staying logged out.');
            }
        }
    }

    const logout = (redirectUri: string) => {
        set_username(undefined);
        window.location.href = redirectUri;
    }

    // the user is logged in if we know their username
    const loggedIn = () => {
        return !!_username;
    }

    // pull this info from session storage
    const username = () => {
        return _username;
    }

    // pull this info directly from realm-export.json (this could also be refactored to use session storage, if desired)
    const hasRole = (roles: string[]) => {
        const userRoles = realmExport.users.find(user => user.username === username())?.realmRoles
        return (userRoles && userRoles.some(role => roles.includes(role))) || false;
    }

    return (
        <AuthContext value={{ login, logout, loggedIn, username, hasRole }}>
            {children}
        </AuthContext>
    );
};