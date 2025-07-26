const authentication_screen = document.getElementById("authentication_screen");
console.log(authentication_screen);

const authentication_username_input = document.getElementById("authentication_username_input");
console.log(authentication_username_input);

const authentication_password_input = document.getElementById("authentication_password_input");
console.log(authentication_password_input);

const authentication_authenticate_button = document.getElementById("authentication_authenticate_button");
console.log(authentication_authenticate_button);

authentication_authenticate_button.addEventListener("click", async () =>
{
	let username_input_value = authentication_username_input.value;
	let password_input_value = authentication_password_input.value;
	if( username_input_value === "" && password_input_value === "")
	{
		alert("Provide an Username and Password value to Sign In.");
		return;
	}

	let response = await fetch("https://eduardoos.com/api/simple_authentication", {
		method: "POST",
		headers: {
			"Content-Type":"application/json"
		},
		body:JSON.stringify({
			username: username_input_value,
			password: password_input_value
		})

	});

	if(response.ok)
	{
		const data = await response.json();
		console.log(data);

		authentication_display_content_for_user(
			data, 
			authentication_screen
		);
	}
	else
	{
		console.error(response.statusText);
		const error_data = await response.json();
		console.log(error);
	}
});

function authentication_display_content_for_user
(
	data, 
	authentication_screen
)
{
	if(data.authenticated)
	{
		authentication_screen.style.display = "none";
	}
	else
	{
		alert(data.message);
	}
} 
