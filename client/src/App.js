import React, {useState} from "react";
import logo from './logo.svg';
import {BrowserRouter, Route, Routes, Navigate} from "react-router-dom";
import './App.css';
import { Register } from './Components/Register';
import { Login } from './Components/Login';
import {Users} from "./Components/Users";

function App() {

  return (
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Navigate to="/login" />}/>
          <Route path="/login" element={<Login/>}/>
          <Route path="/register" element={<Register/>}/>
          <Route path="/users" element={<Users/>}/>
        </Routes>
      </BrowserRouter>
  );
}

export default App;
