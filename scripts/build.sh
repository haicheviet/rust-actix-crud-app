#! /usr/bin/env sh

# Exit in case of error
set -e

# Build the planner stage:
docker build --file Dockerfile \
       --target planner \
       --tag actix-api:planner  .


# Build the compile stage, using cached planner stage:
docker build --file Dockerfile \
       --target builder \
       --cache-from actix-api:planner \
       --cache-from actix-api:builder \
       --tag actix-api:builder  .

# Build the runtime stage, using cached compile stage:
docker build --file Dockerfile \
       --target runtime \
       --cache-from actix-api:runtime \
       --cache-from actix-api:builder \
       --tag actix-api:runtime  .

