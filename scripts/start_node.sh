#!/usr/bin/env bash

docker stop cass1
docker rm cass1
docker run -d -p 9042:9042 --name cass1 cassandra:3.9