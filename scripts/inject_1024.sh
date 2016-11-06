#!/bin/bash

seq 1024 | xargs -I{} -P 1024 curl --data 'message=Plop {}' http://localhost:8080/post
