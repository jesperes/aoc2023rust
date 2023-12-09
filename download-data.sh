#!/bin/bash
# Download inputs and existing solutions

cargo install aoc-cli
TODAY=$(date +%d)

mkdir -p inputs
for day in $(seq 1 "$TODAY"); do
    INPUTFILE=inputs/input$(printf "%02d" "$day").txt
    PUZZLEFILE=inputs/puzzle$(printf "%02d" "$day").md
    if [ ! -f "$INPUTFILE" ] || [ ! -f "$PUZZLEFILE" ]; then
        aoc download -d "$day" -i "$INPUTFILE" -p "$PUZZLEFILE"
    fi
done

(
    echo "{"
    sep=""
    for day in $(seq 1 "$TODAY"); do
        PUZZLEFILE=inputs/puzzle$(printf "%02d" "$day").md
        mapfile -t SOLUTIONS < <(tr -d '`' <"$PUZZLEFILE" | sed -nE 's/Your puzzle answer was (.*)./\1/p')
        echo $sep
        sep=","
        echo -n "   \"$day\": [\"${SOLUTIONS[0]}\", \"${SOLUTIONS[1]}\"]"
    done
    echo
    echo "}"
) | jq >solutions.json
