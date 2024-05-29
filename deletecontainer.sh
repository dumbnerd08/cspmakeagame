#!/usr/bin/bash
for i in $(docker ps -a | grep ago | cut -c 1-12)
do
	docker container rm $i
done
