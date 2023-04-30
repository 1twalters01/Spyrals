Complete fastAPI basic setup
Add routing to fastAPI app
Create demo post function in fastAPI
Make the example leptos post function post to fastAPI
Make fastAPI print the results of the post and return something to show on screen
Connect fastAPI to mysql

Create user model
Create account functions:
* register - add a new user with the is_active field being false
* activate - if token is valid set the chosen user's is_active field to true
* login
* logout
* forgot username - If email is in database then send it an email with the username
* password reset
* password reset token