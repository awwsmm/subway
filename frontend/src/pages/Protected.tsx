import React, { useEffect, useState } from 'react';
import realKeycloak from "../auth/keycloak.ts";
import fakeKeycloak from "../auth/fakeKeycloak.ts";

const Protected: React.FC = () => {
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
                .init({
                    onLoad: 'check-sso',
                    silentCheckSsoRedirectUri: `${window.location.origin}/silent-check-sso.html`
                })
                .then(auth => {

                    console.log(`!auth == ${!auth}, !keycloak.authenticated == ${!keycloak.authenticated}, authenticated == ${authenticated}`)

                    if (!auth && !keycloak.authenticated) {
                        void keycloak.login({redirectUri: window.location.href});
                    } else {
                        setAuthenticated(true);
                    }

                    setLoading(false);
                })
                .catch(err => {
                    console.error('Keycloak init error:', err);
                    setLoading(false);
                });
        }
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
