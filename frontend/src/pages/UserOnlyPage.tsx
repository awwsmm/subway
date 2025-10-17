import React, {useContext, useState} from 'react';
import {Link} from "react-router-dom";
import {AuthContext} from "../auth/AuthContext.tsx";

const UserOnlyPage: React.FC = () => {

    const authContext = useContext(AuthContext);

    const username = authContext?.username();

    const [loggingOut, setLoggingOut] = useState(false);

    // return a null component when logging out to prevent re-rendering this page after the user is reset
    // see https://www.amitmerchant.com/how-to-stop-a-react-component-from-rendering/
    if (loggingOut) {
        return null;
    }

    return (
        <div>
            <h1>🔐 User-Only Page</h1>
            <p>Welcome{username ? `, ${username}!` : `!` }</p>
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
            <div>
                <Link to={'/admin-only'}>
                    <button>
                        Admin-Only Page
                    </button>
                </Link>
            </div>
        </div>
    );
};

export default UserOnlyPage;
