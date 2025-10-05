import './App.css'
import Login from "./pages/Login.tsx";
import Protected from "./pages/Protected.tsx";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import Home from "./pages/Home.tsx";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/login" element={<Login />} />
                <Route path="/protected" element={<Protected />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App
