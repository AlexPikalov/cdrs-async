#!/usr/bin/env bash

docker stop scylla-cdrs
docker rm scylla-cdrs
docker run -d -p 9042:9042 --name scylla-cdrs scylladb/scylla