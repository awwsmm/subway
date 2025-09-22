// src/pages/Protected.tsx
import { useEffect, useState } from 'react';
import realKeycloak from "../auth/keycloak.ts";
import fakeKeycloak from "../auth/fakeKeycloak.ts";

const Protected = () => {
    const [loading, setLoading] = useState(true);
    const [authenticated, setAuthenticated] = useState(false);

    // src/keycloak/keycloak.ts
    const useRealAuth = import.meta.env.VITE_USE_REAL_AUTH === 'true';
    const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

    useEffect(() => {
        keycloak
            .init({ onLoad: 'check-sso', silentCheckSsoRedirectUri: `${window.location.origin}/silent-check-sso.html` })
            .then(auth => {
                if (!auth) {
                    keycloak.login({ redirectUri: window.location.href });
                }
                setAuthenticated(auth);
                setLoading(false);
            })
            .catch(err => {
                console.error('Keycloak init error:', err);
                setLoading(false);
            });
    }, []);

    if (loading) return <p>Loading...</p>;

    if (!authenticated) {
        return <p>Redirecting to login...</p>;
    }

    return (
        <div>
            <h1>🔐 Protected Page</h1>
            <p>Welcome, {keycloak.tokenParsed?.preferred_username}</p>
            <button onClick={() => keycloak.logout({ redirectUri: window.location.origin })}>
                Logout
            </button>
        </div>
    );
};

export default Protected;
