#!/usr/bin/env bash

set -e

mkdir -p build/output

if test -f "build/bin/ec8-assembler"; then

	if [ "$1" == "log" ]; then
		input="$2"
	else 
		input="$1"
	fi

	filename="$(basename -- $1)"
	output="build/output/$filename.c8"
	desc="build/output/$filename.desc"

	./build/bin/ec8-assembler -o "$output" -d "$desc" "$input"

	if [ "$1" == "log" ]; then
		./build/bin/ec8-logging "$output"	
	else
		./build/bin/ec8 "$output"
	fi
else 
	echo "Run build.sh first"
fi