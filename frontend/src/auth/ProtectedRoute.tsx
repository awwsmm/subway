import React, {useContext} from "react";
import {Link, Outlet} from "react-router-dom";
import {AuthContext} from "./AuthContext.tsx";

interface ProtectedRouteProps {
    roles: string[];
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = (props: ProtectedRouteProps) => {

    const authContext = useContext(AuthContext);

    const authorized = authContext?.hasRole(props.roles);

    if (!authorized) {
        // return <Navigate to="/login" replace />; // Redirect to login if not authorized
        return (
            <>
                <div>
                    <p>Unauthorized.</p>
                </div>
                <div>
                    <button>
                        <Link to="/">Back to Home</Link>
                    </button>
                </div>
            </>
        )
    }

    return <Outlet />; // Render nested routes if authenticated
};

export default ProtectedRoute;