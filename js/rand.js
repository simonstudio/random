// http://fent.github.io/randexp.js/
// username:
let usernames = []
for (let i = 0; i <= 200; i++) {
    usernames.push(new RandExp(/^[a-zA-Z]([a-zA-Z0-9]){8,10}$/).gen())
}

usernames.join("\n")


// password: 
let passwords = []
for (let i = 0; i <= 200; i++) {
    passwords.push(new RandExp(/^[a-zA-Z]([a-zA-Z0-9]){8,10}$/).gen())
}