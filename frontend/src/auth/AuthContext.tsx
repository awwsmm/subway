import React, {createContext} from "react";
import {AuthContextInMemoryProvider} from "./AuthContextInMemory.tsx";
import {AuthContextKeycloakProvider} from "./AuthContextKeycloak.tsx";

export interface AuthContext {
    loggedIn: () => boolean;
    login: () => void;
    logout: (redirectUri: string) => void;
    username: () => string | undefined;
    hasRole: (roles: string[]) => boolean;
}

interface AuthContextProps {
    children: React.ReactNode;
    implementation: "in-memory" | "keycloak";
}

export const AuthContext = createContext<AuthContext | undefined>(undefined);

export const AuthContextProvider: React.FC<AuthContextProps> = (props: AuthContextProps) => {

    if (props.implementation === "in-memory") {
        return (
            <AuthContextInMemoryProvider>{props.children}</AuthContextInMemoryProvider>
        );
    } else if (props.implementation === "keycloak") {
        return (
            <AuthContextKeycloakProvider>{props.children}</AuthContextKeycloakProvider>
        );
    } else {
        console.error(`unknown AuthContext implementation: ${props.implementation}`);
    }
}
