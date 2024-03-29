# Rust CRUD API Experiment

## Overview

This project is an experimental CRUD (Create, Read, Update, Delete) API written in Rust. The purpose of this project is to explore building a simple New Post CRUD application in Rust, using the popular [Actix web framework](https://github.com/actix/actix-web) and [Diesel ORM](https://github.com/diesel-rs/diesel).
Mostly for learning purpose

Some highlighted improvements for this experiment:

* Docker multi-stage build for rust using [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) => enable the last layer to only contain one needed binary file and faster build time.
* Leverage [Diesel migration](https://docs.rs/diesel_migrations/latest/diesel_migrations/macro.embed_migrations.html) to migrate database at compile time => enable ship a single executable file.

## Requirements

To run this project, you will need the docker and docker compose:

```bash
docker compose up -d
```

Open http://localhost:8000 (or {docker ip}:8000 on windows) to view it in the browser.


## API Endpoints

The API provides endpoints for basic CRUD operations on a sample entity called Post. Each item has an id, title, body and published.

### Create an Item

```
Endpoint: POST /post
Request Body: JSON object with title, body and optional published fields.
```

### Count all public Post

```
Endpoint: GET /count-post
```

### Get a post by ID

```
Endpoint: GET /post/:id
```

### Publish a post

```
Endpoint: PATCH /post/:id
```

### Delete a post

```
Endpoint: DELETE /post/:id
```