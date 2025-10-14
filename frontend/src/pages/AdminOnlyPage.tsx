import React, {useEffect, useState} from 'react';
import realKeycloak from "../auth/keycloak.ts";
import fakeKeycloak from "../auth/fakeKeycloak.ts";
import {Link} from "react-router-dom";

const AdminOnlyPage: React.FC = () => {
    const [authenticated, setAuthenticated] = useState(false);
    const [admin, setAdmin] = useState(false);
    const [loading, setLoading] = useState(true);

    const useRealAuth = import.meta.env.VITE_SUBWAY_AUTH_MODE === 'docker';
    const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

    useEffect(() => {

        // FIXME username on home page is wiped on refresh -- doesn't affect user-only or admin-only pages

        const realm_access = keycloak.tokenParsed?.realm_access;
        const roles = realm_access?.roles;
        const indexOf = roles?.indexOf("admin");
        const isAdmin = indexOf !== undefined && indexOf > -1;

        if (keycloak.authenticated) {
            setAuthenticated(true);
            setLoading(false);
            setAdmin(isAdmin);
        } else {
            keycloak
                .init({onLoad: "login-required"}) // or "check-sso" if you want silent login
                .then((auth) => {

                    const realm_access = keycloak.tokenParsed?.realm_access;

                    console.log("keycloak.tokenParsed?.realm_access == " + JSON.stringify(realm_access));

                    const roles = realm_access?.roles;

                    console.log("realm roles == " + JSON.stringify(roles));

                    const indexOf = roles?.indexOf("admin");

                    console.log("indexOf " + JSON.stringify(indexOf));

                    const isAdmin = indexOf !== undefined && indexOf > -1;

                    console.log("isAdmin " + isAdmin);

                    setAdmin(isAdmin);
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

    if (!admin) {
        return (
            <div>
                <div>
                    <p>User is not admin.</p>
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
            <h1>🔐 Admin-Only Page</h1>
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

export default AdminOnlyPage;
