#! /usr/bin/env bash
set -euo pipefail

AOC_SESSION=$(cat aoc_cookie.txt)

while [ "$#" -gt 0 ]; do
  AOC_DAY="$1"
  shift

  curl \
      --cookie session="${AOC_SESSION}" \
      https://adventofcode.com/2024/day/"${AOC_DAY}"/input > input/day_"${AOC_DAY}".txt
done


