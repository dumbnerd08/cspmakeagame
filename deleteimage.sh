#!/usr/bin/bash
#60 images
for i in {1..60}
do
	docker image rm $(docker image list | grep ago | cut -c 42-53 | sed -n $i'p')
done
ids=$(docker image list | grep ago | cut -c 42-53 | sed -n '60p')

echo $ids
