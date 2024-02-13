# Rust-Data-Validation-API

## Setup

You’ll need:

- Mongo DB
- Rust for the backend (cargo watch for working on the project while running)
- Optional: Node for the fronted

Create a .env file inside the backend directory and place the MongoDB connection string starting with `MONGOURI=` like this

```jsx
MONGOURI=mongodb://localhost:27017/?readPreference=primary&ssl=false&directConnection=true
```

Cd into backend directory and run:

`cargo run`

Or use `cargo watch -x run` if you want to make changes while running the project.

## Endpoints

### Add User

To add an user:

`localhost:8080/user`

Send a `POST` request with a json object in this format:

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "age": "22",
  "pensum": "77",
  "location": "Zurich",
  "occupation": "Software Engineer",
  "ahv_nr": "756.1718.4457.72"
}
```

You can generate valid numbers from this website:

[https://www.uhutools.ch/ahv-nummer/de/](https://www.uhutools.ch/ahv-nummer/de/)

### Get User

You can get an user from:

`localhost:8080/user/<id>`

Replace `"id"` with the actual id.

### Delete User

Send a `DELETE` request to the endpoint:

`localhost:8080/user/<id>`

### Update User

Send a PUT request to the endpoint:

`localhost:8080/user/<id>`

Provide one of the following fields:

```jsx
{
  "first_name": "",
  "last_name": "",
  "location": "",
  "occupation": "",
  "ahv_nr": ""
}
```

# Frontend

Cd into frontend directory. Create a file called `.env.local` and add the following:

`NEXT_PUBLIC_MONGO_API=http://localhost:8080`

Then, run:

`npm install`

Followed by:

`npm run dev`

# XML Config

In the root directory of the backend folder, you'll find a file called `config.xml`. You can make some changes to the file and setup Validation rules at runtime.
# Rust-Data-Validation-API
