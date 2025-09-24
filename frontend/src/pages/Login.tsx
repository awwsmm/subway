import React, { useEffect, useState } from "react";
import realKeycloak from "../auth/keycloak";
import fakeKeycloak from "../auth/fakeKeycloak.ts";

const Login: React.FC = () => {
    const [authenticated, setAuthenticated] = useState(false);
    const [loading, setLoading] = useState(true);

    const useRealAuth = import.meta.env.VITE_SUBWAY_AUTH_MODE === 'docker';
    const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

    useEffect(() => {
        if (keycloak.authenticated) {
            setAuthenticated(true);
            setLoading(false);
        } else {
            keycloak
                .init({onLoad: "login-required"}) // or "check-sso" if you want silent login
                .then((auth) => {
                    setAuthenticated(auth);
                    setLoading(false);
                })
                .catch((err) => {
                    console.error("Keycloak init error:", err);
                    setLoading(false);
                });
        }
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
