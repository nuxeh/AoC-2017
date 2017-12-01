#!/bin/bash
count=0

while read -p "test [a/b]> " t; do
	if [[ $t == "a" ]] || [[ $t == "b" ]]; then
		break
	fi
	echo "invalid test"
done

while read -p "input> " i && [[ $i != "q" ]]
do
	mkdir -p inputs_$t
	path=inputs_$t/t$count.txt
	echo $i > $path
	echo "$i -> $path"

	read -p "result> " r

	if [[ $r != "" ]]; then
		path=inputs_$t/r$count.txt
		echo $r > $path
		echo "$r -> $path"
	fi

	count=$[count + 1]
done

