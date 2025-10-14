import React, {useEffect, useState} from 'react';
import realKeycloak from "../auth/keycloak.ts";
import fakeKeycloak from "../auth/fakeKeycloak.ts";
import {Link} from "react-router-dom";

const UserOnlyPage: React.FC = () => {
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

                    console.log("keycloak.tokenParsed?.realm_access == " + JSON.stringify(keycloak.tokenParsed?.realm_access));

                    console.log("realm roles == " + JSON.stringify(keycloak.tokenParsed?.realm_access?.roles));

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
        return (
            <div>
                <div>
                    <p>Unable to authenticate.</p>
                </div>
                <div>
                    <button>
                        <Link to="/">Back to Home</Link>
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div>
            <h1>🔐 User-Only Page</h1>
            <p>Welcome, {keycloak.tokenParsed?.preferred_username}</p>
            <div>
                <button onClick={() => keycloak.logout({ redirectUri: window.location.origin })}>
                    Logout
                </button>
            </div>
            <div>
                <button>
                    <Link to="/">Back to Home</Link>
                </button>
            </div>
        </div>
    );
};

export default UserOnlyPage;
