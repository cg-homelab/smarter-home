import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./app";
import User from "./user";
import Charts from "./charts";
import Settings from "./settings";
import UserLogin from "./user-login";
import NavBar from "./components/navbar";
import "./global.css";


ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <div className="container">
        <NavBar />

        <Routes>
          <Route path="/" element={<App />} />
          <Route path="/user" element={<User />} />
          <Route path="/analytics" element={<Charts />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/login" element={<UserLogin />} />
        </Routes>
        
      </div>
    </BrowserRouter>
  </React.StrictMode>
);
