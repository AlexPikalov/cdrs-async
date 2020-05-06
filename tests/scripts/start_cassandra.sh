#!/usr/bin/env bash

docker stop cassandra-cdrs
docker rm cassandra-cdrs
docker run -d -p 9042:9042 --name cassandra-cdrs cassandra:3.9