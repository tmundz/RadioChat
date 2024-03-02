import React, { createContext, useEffect, useContext, useState } from "react";

function getCookie(cookie) {
  const allCookies = document.cookie.split(";");
  for (const cookie of allCookies) {
    const [name, value] = cookie.split("=").map((part) => part.trim());

    if (name === "session") {
      return value;
    }
  }
  return null;
}

function parseUserFromSession(sessionData) {
  try {
    return JSON.parse(sessionData);
  } catch (err) {
    console.error("Error parsing session data:", err);
    return null;
  }
}

const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [curUser, setCurUser] = useState(null);

  useEffect(() => {
    const checkExistingSession = async () => {
      try {
        const sessionData = getCookie("auth_session");
        if (sessionData) {
          const userData = parseUserFromSession(sessionData);
          setCurUser(userData);
        }
      } catch (error) {
        console.error("Error Checking Session:", error);
      }
    };
    checkExistingSession();
  }, []);

  const authenticateUser = async (userData) => {
    setCurUser(userData);
  };

  const logout = async () => {
    setCurUser(null);
  };

  return (
    <AuthContext.Provider value={{ curUser, authenticateUser, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);
