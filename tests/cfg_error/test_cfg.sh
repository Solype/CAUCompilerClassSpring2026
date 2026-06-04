#!/bin/bash

file=$1

for file in *.cfg; do
    echo "===================================="
    echo "Testing: $file"
    echo "===================================="

    $1 "$file"

    echo
done

