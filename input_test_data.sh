#!/bin/bash
count=0
while read -p "input> " i && [[ $i != "q" ]] ; do
	echo $i > t$count.txt
	echo "$i -> t$count.txt"

	read -p "result> " r
	echo $r > r$count.txt
	echo "$r -> r$count.txt"

	count=$[count + 1]
done

