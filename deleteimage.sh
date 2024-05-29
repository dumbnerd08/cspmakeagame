#!/usr/bin/bash
text=$(docker image list)
search="IMAGE ID"
rest=${text#*$search}
index=$(( ${#text} - ${#rest} - ${#search} ))
higherIndex=$(( $index + 11 ))
num=$(docker image list | grep ago | cut -c $index-$higherIndex)
for i in $num
do
        docker image rm $i
done
