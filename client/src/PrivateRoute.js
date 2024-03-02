import React from 'react';
import { Navigate, Outlet } from 'react-router-dom';
import { useAuth } from './authContext'; // Adjust the path as needed

const PrivateRoute = () => {
  const { currentUser } = useAuth();
  return currentUser ? <Outlet /> : <Navigate to="/login" />;
};

export default PrivateRoute;

