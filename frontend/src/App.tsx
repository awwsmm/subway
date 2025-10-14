import './App.css'
import LoginPage from "./pages/LoginPage.tsx";
import AdminOnlyPage from "./pages/AdminOnlyPage.tsx";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import HomePage from "./pages/HomePage.tsx";
import UserOnlyPage from "./pages/UserOnlyPage.tsx";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/login" element={<LoginPage />} />
                <Route path="/admin-only" element={<AdminOnlyPage />} />
                <Route path="/user-only" element={<UserOnlyPage />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App
