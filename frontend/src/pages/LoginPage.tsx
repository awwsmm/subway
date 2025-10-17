import React, {useContext, useState} from "react";
import {Link} from "react-router-dom";
import {AuthContext} from "../auth/AuthContext.tsx";

const LoginPage: React.FC = () => {

    const [loggingOut, setLoggingOut] = useState(false);

    // return a null component when logging out to prevent re-rendering this page after the user is reset
    // see https://www.amitmerchant.com/how-to-stop-a-react-component-from-rendering/
    if (loggingOut) {
        return null;
    }

    const authContext = useContext(AuthContext);

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
                <button onClick={() => { setLoggingOut(true); authContext?.logout(window.location.origin) }}>
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
