#!/bin/bash

if ! which cloc; then
	echo "\`cloc\` not installed, installing..."
	sudo apt -y install cloc
else
	cloc .
fi
