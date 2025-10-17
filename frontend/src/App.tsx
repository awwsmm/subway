import './App.css'
import LoginPage from "./pages/LoginPage.tsx";
import AdminOnlyPage from "./pages/AdminOnlyPage.tsx";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import HomePage from "./pages/HomePage.tsx";
import UserOnlyPage from "./pages/UserOnlyPage.tsx";
import {AuthContextInMemoryProvider} from "./auth/AuthContextInMemory.tsx";
import ProtectedRoute from "./auth/ProtectedRoute.tsx";

function App() {
    return (
        <AuthContextInMemoryProvider>
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
        </AuthContextInMemoryProvider>
    );
}

export default App
