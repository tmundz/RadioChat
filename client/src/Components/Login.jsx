import React, {useState} from "react";
import { useNavigate } from "react-router-dom";

export const Login = (props) => {
    const navigate = useNavigate();
    const [user_name, setUserName] = useState('');
    const [password, setPassword] = useState('');
    const [errorMsg, setErrorMsg] = useState('');

    // POST check login 
    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await fetch('/api/auth/login', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({
                    user_name,
                    password
                })
            });

            const data = await response.json();
            if (!response.ok) {
                setErrorMsg(data.message || "Unknown error occured");
                console.error("Error during submission: ", data.message);
                alert(data.message);
                throw new Error(data.message);
            }

            console.log('Submission successful', data);
            setErrorMsg("");
            navigate("/users");
        } catch (error) {
            console.error("Error during form submision: ", error);
        }
    };

    // Get  page
    const handleReg = (e) => {
        navigate("/register");
    }
    
    return (
        <>
        <div className="auth-form-container">
            <form onSubmit={handleSubmit}>

                <label htmlFor="user_name">User Name</label>
                <input value={user_name} onChange={(e) => setUserName(e.target.value)} type="user_name" id="user_name" name="user_name"/>
                <label htmlFor="Password">Password</label>
                <input value={password} onChange={(e) => setPassword(e.target.value)} type="password" id="password" name="password"/>
                <button type="submit">Submit</button>
            </form>
            <button onClick={() => handleReg()}>Dont have an account? Register here.</button>
        </div>
        </>
    )
}