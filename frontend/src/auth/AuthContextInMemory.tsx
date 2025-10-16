import React, {createContext, useEffect, useState} from "react";
import type AuthContext from "./AuthContext.ts";

export const AuthContextInMemory = createContext<AuthContext | undefined>(undefined);

export const AuthContextInMemoryProvider = ({ children }: { children: React.ReactNode }) => {
    const users = ["admin", "bob", "clara"];

    const [user, setUser] = useState(() => {
        const savedData = sessionStorage.getItem('myAppStateKey');
        return savedData && savedData !== "" ? JSON.parse(savedData) : undefined;
    });

    // Use useEffect to save state to sessionStorage whenever 'user' changes
    useEffect(() => {
        sessionStorage.setItem('myAppStateKey', user ? JSON.stringify(user) : "");
    }, [user]); // Dependency array: effect runs when 'user' changes

    const init = () => {
        // in-memory init() does nothing
    }

    const loggedIn = () => {
        console.log(`loggedIn called -- user = ${user}`);
        return !!user;
    }

    const login = () => {
        console.log(`login called -- user = ${user}`);
        if (user) {
            return true

        } else {
            const username = prompt(`Enter username to login (e.g., ${users.join(", ")}):`);
            if (username && users.includes(username)) {
                setUser(username);
                return true

            } else {
                alert('Invalid user. Staying logged out.');
                return false
            }
        }
    }

    const logout = (redirectUri: string) => {
        window.location.href = redirectUri;
        setUser(undefined);
    }

    const username = () => {
        return user
    }

    const hasRole = () => {
        return true;
    }

    return (
        <AuthContextInMemory value={{ init, loggedIn, login, logout, username, hasRole }}>
            {children}
        </AuthContextInMemory>
    );
};