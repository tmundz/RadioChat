import React from "react";
import { BrowserRouter, Route, Routes, Navigate } from "react-router-dom";
import PrivateRoute from "./PrivateRoute";
import "./App.css";
import { Register } from "./Components/RegisterPage";
import { Login } from "./Components/LoginPage";
import { Home } from "./Components/HomePage";

function App() {
  return (
    <BrowserRouter>
      {/* public routes*/}
      <Routes>
        <Route path="/" element={<Navigate to="/login" />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route element={<PrivateRoute />}>
          <Route path="/home/:username" element={<Home />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
