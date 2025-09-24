import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import User from "./User";
import Charts from "./Charts";
import Settings from "./Settings";
import UserLogin from "./User-Login";
import NavBar from "./components/navbar";
import "./Global.css";


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
