# Instagram Clone - RESTful API

## Overview
This is a RESTful API for an Instagram clone. It is built using Rust, sea_orm as ORM, and actix-web as the web framework. The database used is MySQL.

## Deployment
Currently, the API is not deployed yet. It will be deployed to fly.io once I implement the observability features.

## Features
- [ ] User registration
- [ ] User login
- [ ] User logout
- [ ] get user profile
- [ ] update user profile
- [ ] User profile delete
- [ ] upload/update user profile picture
- [ ] follow user
- [ ] unfollow user
- [ ] get followers
- [ ] get following
- [ ] create post
- [ ] get post
- [ ] get posts from following
- [ ] delete post
- [ ] update post
- [ ] like post
- [ ] unlike post
- [ ] comment on post
- [ ] delete comment
- [ ] create story
- [ ] get story
- [ ] delete story
- [ ] add post to favorites
- [ ] remove post from favorites
- [ ] get posts from favorites
- [ ] add post to bookmarks
- [ ] remove post from bookmarks
- [ ] get posts from bookmarks

## Getting Started
### Prerequisites
- Rust (I'm using 1.74.0)
- MySQL

### Installation
1. Clone the repo
   ```sh
   git clone 
    ```
2. Run cargo check to install dependencies
    ```sh
    cargo check
    ```
3. Create a `.env` file in the root directory and add the following environment variables
   ```sh
   DATABASE_URL=mysql://<username>:<password>@<host>:<port>/<database_name>
   ```
4. Install sea_orm cli to run migrations and generate entities
   ```sh
   cargo install sea-orm-cli
   ```
5. Run the migrations
   ```sh
    sea migration up
    ```
6. Create JWT private key
   ```sh
   # Generate a RSA private key
   openssl genrsa -out privatekey.pem 2048
   ```
   after that encode the private key to base64
   ```sh
    openssl base64 -in privatekey.pem
    ```
   copy the encoded private key and paste it in the `.env` file like so:
   ```
   APP_JWT__PRIVATE_KEY=<encoded_private_key>
   ```
7. Create JWT public key
    ```sh
    # Generate a RSA public key
    openssl rsa -in privatekey.pem -out publickey.pem -pubout -outform PEM
    ```
   after that encode the public key to base64
    ```sh
     openssl base64 -in publickey.pem
    ```
   copy the encoded public key and paste it in the `.env` file like so:
    ```
    APP_JWT__PUBLIC_KEY=<encoded_public_key>
    ```
8. Install bunyan formatter for logging (optional)
    ```sh
    cargo install bunyan
    ```
9. Run the app
    ```sh
    cargo run
    ```
   with bunyan
    ```sh
    cargo run | bunyan
    ```
   or with cargo-watch
    ```sh
    cargo watch -q -c -x run
    ```
   with bunyan
    ```sh
    cargo watch -q -c -x run | bunyan
    ```

### Testing
```sh
cargo tests
```

## Migration
Run migration. Make sure to install `sea-orm-cli` first
1. Up
    ```sh
    sea migration up
    ```
2. Down
    ```sh
    sea migration down
    ```

## Entity
Generate entity from database schema. Make sure to install `sea-orm-cli` first. It uses `DATABASE_URL` from .env file
```sh
sea generate entity -l -o entity/src
```