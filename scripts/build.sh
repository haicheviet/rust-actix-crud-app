#! /usr/bin/env sh

# Exit in case of error
set -e

# Build the planner stage:
docker build --file Dockerfile \
       --target planner \
       --tag $DOCKER_IMAGE_APP:planner  .


# Build the compile stage, using cached planner stage:
docker build --file Dockerfile \
       --target builder \
       --cache-from $DOCKER_IMAGE_APP:planner \
       --cache-from $DOCKER_IMAGE_APP:builder \
       --tag $DOCKER_IMAGE_APP:builder  .

# Build the runtime stage, using cached compile stage:
docker build --file Dockerfile \
       --target runtime \
       --cache-from $DOCKER_IMAGE_APP:runtime \
       --cache-from $DOCKER_IMAGE_APP:builder \
       --tag $DOCKER_IMAGE_APP:runtime  .

