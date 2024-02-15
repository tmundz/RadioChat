import React, {useState} from "react";
import { useNavigate } from "react-router-dom";

export const Login = (props) => {
    const navigate = useNavigate();
    const [user_name, setUserName] = useState('');
    const [password, setPassword] = useState('');

    // POST check login 
    const handleSubmit = (e) => {
        //if  login success 
        // need to set up the backend calls
        //get pages
        navigate("/users");
        console.log(user_name);
    };

    // Get  page
    const handleReg = (e) => {
        navigate("/register");
    }
    
    return (
        <>
        <div className="auth-form-container">
            <form>
                <label for="user_name">User Name</label>
                <input value={user_name} onChange={(e) => setUserName(e.target.value)} type="user_name" id="user_name" name="user_name"/>
                <label for="Password">Password</label>
                <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" id="password" name="password"/>
                <button onClick={() => handleSubmit()}>Submit</button>
            </form>
            <button onClick={() => handleReg()}>Dont have an account? Register here.</button>
        </div>
        </>
    )
}