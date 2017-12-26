#!/bin/bash

basedir=$(pwd -P | sed 's/\(AoC_20..\).*$/\1/')

if ! which cloc &> /dev/null; then
	echo "\`cloc\` not installed, installing..."
	sudo apt -y install cloc
fi

printf "\n$PWD\n\n"
if ! [[ "$PWD" == "$basedir" ]]; then
	cloc .
fi

printf "$basedir \n\n"
cloc $basedir
