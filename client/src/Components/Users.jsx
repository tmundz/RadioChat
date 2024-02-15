// Components/Users.js
import React, {useEffect, useState} from "react";
export const Users = () => {
    const [users, setUsers] = useState([]);

    useEffect(() => {
        const fetchUsers = async () => {
            const response = await fetch('/api/get_users');
            if (!response.ok) {
                console.error('Failed to fetch users');
                return;
            }
            const data = await response.json();
            setUsers(data);
        };
        fetchUsers();
    }, []);
  return (
    <div>
      <h1>Users </h1>
      {users.map(user => (
      <div key={user.uid}>
        <p>{user.uid}:{user.user_name}:{user.email}</p>
      </div>))}
    </div>
  );
};

