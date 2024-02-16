import React, {useState} from "react";
import { useNavigate } from "react-router-dom";

export const Register = () => {
    const navigate = useNavigate();
    const [user_name, setUserName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');


    // POST create account
    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await fetch('/api/auth/register', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({
                    user_name,
                    email,
                    password
                })
            });
            if (!response.ok) {
                throw new Error('Username or email is already in use')
            }

            const data = await response.json();
            console.log('Submission successful', data);
            navigate("/users")
        } catch (error) {
            console.error("Error during form submision: ", error);
        }
    };

    const handleLogin = (e) => {
        navigate("/login");
        console.log(user_name);
    }
    
    return (
        <>
        <div className="register-form-container">
            <form onSubmit={handleSubmit}>
                <label htmlFor="user_name">User Name</label>
                <input value={user_name} onChange={(e) => setUserName(e.target.value)} type="text" id="user_name" name="user_name"/>
                <label htmlFor="user_name">Email</label>
                <input value={email} onChange={(e) => setEmail(e.target.value)} type="email" id="email" name="email"/>
                <label htmlFor="password">Password</label>
                <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" id="password" name="password"/>
                <button type="submit">Submit</button>
            </form>
            <button onClick={() => handleLogin()}>Have an account? Login here.</button>
        </div>
        </>
    );
};