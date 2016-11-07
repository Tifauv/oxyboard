#!/bin/bash

if [ $# -ne 1 ]
then
	echo "== Oxyboard post injector"
	echo "Usage: ./inject.sh <number_of_posts>"
	exit
fi

num_posts=$1
seq $num_posts | xargs -I{} -P 1024 curl --data 'message=Plop {}' http://localhost:8080/post
