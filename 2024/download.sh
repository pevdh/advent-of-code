#! /usr/bin/env bash
# Usage: ./download.sh [day] [day] [day] ...
# pro-tip: use shell expansion to download multiple days at once: ./download.sh {1..25}

set -euo pipefail

AOC_SESSION=$(cat aoc_cookie.txt)

while [ "$#" -gt 0 ]; do
  AOC_DAY="$1"
  shift

  curl \
      --cookie session="${AOC_SESSION}" \
      https://adventofcode.com/2024/day/"${AOC_DAY}"/input > input/day_"${AOC_DAY}".txt
done


