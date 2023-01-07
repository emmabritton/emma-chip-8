#!/usr/bin/env bash

set -e

if test -f "build/bin/ec8"; then

	if [ "$1" == "log" ]; then
		./build/bin/ec8-logging "$2"
	else 
		./build/bin/ec8 "$1"
	fi
	
else 
	echo "Run build.sh first"
fi