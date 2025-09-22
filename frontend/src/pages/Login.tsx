// src/pages/Login.tsx
import React, { useEffect, useState } from "react";
import realKeycloak from "../auth/keycloak";
import fakeKeycloak from "../auth/fakeKeycloak.ts";

const Login: React.FC = () => {
    const [authenticated, setAuthenticated] = useState(false);
    const [loading, setLoading] = useState(true);

    // src/keycloak/keycloak.ts
    const useRealAuth = import.meta.env.VITE_USE_REAL_AUTH === 'true';
    const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

    useEffect(() => {
        keycloak
            .init({ onLoad: "login-required" }) // or "check-sso" if you want silent login
            .then((auth) => {
                setAuthenticated(auth);
                setLoading(false);
            })
            .catch((err) => {
                console.error("Keycloak init error:", err);
                setLoading(false);
            });
    }, []);

    if (loading) return <p>Loading...</p>;

    if (!authenticated) {
        return <p>Unable to authenticate.</p>;
    }

    return (
        <div>
            <h1>Welcome, {keycloak.tokenParsed?.preferred_username}!</h1>
            <p>You are logged in via Keycloak.</p>
            <button onClick={() => keycloak.logout({ redirectUri: window.location.origin })}>
                Logout
            </button>
        </div>
    );
};

export default Login;
