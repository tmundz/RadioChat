import React, {useState} from "react";
import { useNavigate } from "react-router-dom";

export const Register = () => {
    const navigate = useNavigate();
    const [user_name, setUserName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');


    // POST create account
    const handleSubmit = (e) => {
        e.preventDefault();
        //if Create success 
        // need to set up the backend calls
        //get pages
        navigate("/users");
        console.log(user_name);
    };

    const handleLogin = (e) => {
        navigate("/login");
        console.log(user_name);
    }
    
    return (
        <>
        <div className="register-form-container">
            <form>
                <label for="user_name">User Name</label>
                <input value={user_name} onChange={(e) => setUserName(e.target.value)} type="user_name" id="user_name" name="user_name"/>
                <label for="user_name">Email</label>
                <input value={email} onChange={(e) => setEmail(e.target.value)} type="email" id="email" name="email"/>
                <label for="Password">Password</label>
                <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" id="password" name="password"/>
                <button type="submit">Submit</button>
            </form>
            <button onClick={() => handleLogin()}>Have an account? Login here.</button>
        </div>
        </>
    )
}