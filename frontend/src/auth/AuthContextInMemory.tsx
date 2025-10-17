import React, {createContext, useEffect, useState} from "react";
import type AuthContext from "./AuthContext.ts";
import realmExport from "../../../keycloak/realm-export.json";

export const AuthContextInMemory = createContext<AuthContext | undefined>(undefined);

export const AuthContextInMemoryProvider = ({ children }: { children: React.ReactNode }) => {

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
            if (realmExport) {
                setInitialized(true);
            }
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

            const users = realmExport.users.map(each => each.username);
            const username = prompt(`Enter username to login (e.g., ${users.join(", ")}):`);

            if (username && users.includes(username)) {
                setUser(username);

            } else {
                alert('Invalid user. Staying logged out.');
            }
        }
    }

    const logout = (redirectUri: string) => {
        setUser(undefined);
        window.location.href = redirectUri;
    }

    const username = () => {
        return user
    }

    const hasRole = (roles: string[]) => {
        const userRoles = realmExport.users.find(user => user.username === username())?.realmRoles
        return (userRoles && userRoles.some(role => roles.includes(role))) || false;
    }

    return (
        <AuthContextInMemory value={{ loggedIn, login, logout, username, hasRole }}>
            {children}
        </AuthContextInMemory>
    );
};