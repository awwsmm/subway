import './App.css'
import LoginPage from "./pages/LoginPage.tsx";
import AdminOnlyPage from "./pages/AdminOnlyPage.tsx";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import HomePage from "./pages/HomePage.tsx";
import UserOnlyPage from "./pages/UserOnlyPage.tsx";
import ProtectedRoute from "./auth/ProtectedRoute.tsx";
import {AuthContextProvider} from "./auth/AuthContext.tsx";

function App() {
    return (
        <AuthContextProvider implementation={import.meta.env.VITE_SUBWAY_AUTH_MODE}>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route element={<ProtectedRoute roles={["admin"]} />}>
                        <Route path="/admin-only" element={<AdminOnlyPage />} />
                    </Route>
                    <Route element={<ProtectedRoute roles={["user"]} />}>
                        <Route path="/user-only" element={<UserOnlyPage />} />
                    </Route>
                </Routes>
            </BrowserRouter>
        </AuthContextProvider>
    );
}

export default App
