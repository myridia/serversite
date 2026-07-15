#!/bin/bash

docker rm --force `docker ps -qa`
docker volume rm $(docker volume ls -q --filter dangling=true)


if [ "$1" == "all" ] || [ $# -gt 1 ]; then
  echo "clean all including images"
  docker rmi --force `docker images -aq`    
fi

