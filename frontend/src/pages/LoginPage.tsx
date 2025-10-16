import React, {useContext} from "react";
// import realKeycloak from "../auth/keycloak";
// import fakeKeycloak from "../auth/fakeKeycloak.ts";
import {Link} from "react-router-dom";
import {AuthContextInMemory} from "../auth/AuthContextInMemory.tsx";

const LoginPage: React.FC = () => {
    // const [authenticated, setAuthenticated] = useState(false);
    // const [loading, setLoading] = useState(true);

    // const useRealAuth = import.meta.env.VITE_SUBWAY_AUTH_MODE === 'docker';
    // const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

    // useEffect(() => {
    //     if (keycloak.authenticated) {
    //         setAuthenticated(true);
    //         setLoading(false);
    //     } else {
    //         keycloak
    //             .init({onLoad: "login-required"}) // or "check-sso" if you want silent login
    //             .then((auth) => {
    //                 setAuthenticated(auth);
    //                 setLoading(false);
    //             })
    //             .catch((err) => {
    //                 console.error("Keycloak init error:", err);
    //                 setLoading(false);
    //             });
    //     }
    // }, []);

    const authContext = useContext(AuthContextInMemory);

    // authContext?.init();


    // if (loading) return <p>Loading...</p>;
    //
    // if (!authenticated) {
    //     return <p>Unable to authenticate.</p>;
    // }

    if (!authContext?.loggedIn()) {
        return (
            <div>
                <h1>Welcome!</h1>
                <p>You can log in by clicking the button below.</p>
                <div>
                    <button onClick={() => authContext?.login()}>
                        Login
                    </button>
                </div>
                <div>
                    <button>
                        <Link to="/">Back to Home</Link>
                    </button>
                </div>
            </div>
        )
    }

    return (
        <div>
            <h1>Welcome, {authContext?.username()}!</h1>
            <p>You are logged in. You can log out by clicking the button below.</p>
            <div>
                <button onClick={() => authContext?.logout(window.location.origin)}>
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

export default LoginPage;
