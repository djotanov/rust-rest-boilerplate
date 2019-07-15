# Rust REST Api Boilerplate

[![dependency status](https://deps.rs/repo/github/djotanov/rust-rest-boilerplate/status.svg)](https://deps.rs/repo/github/djotanov/rust-rest-boilerplate)
[![lines of code](https://tokei.rs/b1/github/djotanov/rust-rest-boilerplate)](https://github.com/djotanov/rust-rest-boilerplate)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/djotanov/rust-rest-boilerplate/blob/master/LICENSE)

## About
This is a REST api boilerplate project made using some of the best available tools for rust. Requires 1.31+ stable rust.

## Technologies
- `actix web` for web server
- `diesel` for ORM
- `log4rs` for logging
- `failure` for error handling
- `config-rs` for hierarchical configuration management
- `validator` for validation

## Setup



Install cargo-generate, diesel-cli and cargo-watch

    cargo install cargo-generate
    cargo install diesel_cli
    cargo install cargo-watch
    
Clone this repo using cargo-generate

    cargo generate --git https://github.com/djotanov/rust-rest-template.git --name myproject

Install postgres database and make sure that local user is admin and it should use "peer" or "trust" authentication methods (see `pg_hba.conf`)
Than run the application

    cargo run

Alternatively, you can run the application with postgres running in docker

    docker run -p 5431:5432 -e POSTGRES_PASSWORD='postgres' postgres:alpine
    PROFILE=docker cargo run

Yet another way is to build docker image with the application and use docker compose to start application and postgres

    docker build -t {{crate_name}} .
    docker-compose up

## Development

To create a new endpoint (or group of endpoints):

- Append new service to ServiceConfig in `routes.rs` and define handler bindings
- Implement handler (in `handler` folder)
- Implement domain model (in `model` folder)
- Implement service (in `service` folder)
- Create database migrations (in `migrations` folder)
- Implement repository (in `repository` folder)

Now you can launch `watch.sh` script which will run the migrations and tests and start the application on all code changes.

    ./watch.sh
    
or, if you are running postgres in docker:

    PROFILE=docker ./watch.sh
    
To rename the project, just replace all occurances of `boilerplate` in 