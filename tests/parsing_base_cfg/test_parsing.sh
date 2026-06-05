#!/bin/bash

BIN="$1"

tests=0
success=0

run_test() {
    local file="$1"
    local expected="$2"

    ((tests++))

    echo "===================================="
    echo "Testing: $file"
    echo "===================================="

    "$BIN" "$file"
    status=$?

    echo "===================================="

    if [ "$status" -eq "$expected" ]; then
        echo "PASS (exit code $status)"

        ((success++))
    else
        echo "FAIL (expected $expected, got $status)"
    fi

    echo
}

for file in success/*.txt; do
    run_test "$file" 0
done

for file in errors/*.txt; do
    run_test "$file" 1
done

echo "Passed $success/$tests tests"

if [ "$tests" -ne "$success" ]; then
    exit 1
fi

exit 0