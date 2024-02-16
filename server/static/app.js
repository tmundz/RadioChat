document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('registerForm').addEventListener('submit', async (event) => {
        event.preventDefault();

        const formData = {
            user_name: document.getElementById('user_name').value,
            email: document.getElementById('email').value,
            password: document.getElementById('password').value,
        };

        try {
            const response = await fetch('http://127.0.0.1:8080/add_user', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData),
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const result = await response.json();
            console.log(result);
            alert('User created successfully');
            // Redirect or clear form here as needed
        } catch (error) {
            console.error('Error creating user:', error);
            alert('Failed to create user');
        }
    });
});

