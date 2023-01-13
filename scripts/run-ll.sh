#!/usr/bin/env bash

set -e

mkdir -p build/output

if test -f "build/bin/ec8-ll-compiler"; then

	if [ "$1" == "log" ]; then
		input="$2"
	else 
		input="$1"
	fi

	filename=$(basename "${input%.*}")
	asm_output="build/output/$filename.eca"
	bin_output="build/output/$filename.c8"
	desc="build/output/$filename.desc"

	./build/bin/ec8-ll-compiler -o "$asm_output" "$input"
	./build/bin/ec8-assembler -o "$bin_output" -d "$desc" "$asm_output"


	if [ "$1" == "log" ]; then
		./build/bin/ec8-logging "$bin_output"	
	else
		./build/bin/ec8 "$bin_output"
	fi
else 
	echo "Run build.sh first"
fi