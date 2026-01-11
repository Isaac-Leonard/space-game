# Space Game
This is a simple text based space rpg / rts game that I have been working on.
It uses the loco framework with the backend being written in rust and the frontend using typescript with react and Tanstack router.
## Running locally
To run locally you will need nodejs installed with npm, rustup installed with cargo and a local email catcher such as mail hog or mailtutan.
For more information on local email catchers look at the loco mailers [documentation](https://loco.rs/docs/processing/mailers/).
Once these are all installed in separate terminal tabs run the following commands:
```
mailtutan # Or what ever other mail catcher you have installed
cargo run start
npm run dev
```
This final command should launch your browser to the home page of the application.
In a separate browser tab you will probably want to open the email catcher you have ran using the http url it specifies.
You can then navigate to the register page of the application and proceed to create an account.
An email will be sent that will show up in the mail catcher with a link you can click on to create your account.
