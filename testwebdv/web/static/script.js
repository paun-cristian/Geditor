document.addEventListener("DOMContentLoaded", function() {
    const form = document.querySelector("registerForm");

    form.addEventListener("submit", async function(event) {
        event.preventDefault(); // Prevent default form submission

        const formData = new FormData(form);
        const data = new URLSearchParams(formData);

        try {
            const response = await fetch("/register", {
                method: "POST",
                body: data,
            });

            if (response.ok) {
                const text = await response.text();
                document.querySelector("#response").innerHTML = text;
            } else {
                console.error("Error:", response.statusText);
            }
        } catch (error) {
            console.error("Request failed", error);
        }
    });
});

document.getElementById('register-button').addEventListener('click', function() {
    document.getElementById('register-text').textContent = 'Create account';

});

document.getElementById('login-button').addEventListener('click', function() {
    document.getElementById('register-text').textContent = 'Login';
});