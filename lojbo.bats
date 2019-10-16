#!/usr/bin/env bats

@test "root word file exist" {
  find ./data/gismu.txt
}

@test "filter root words" {
  len=$(cat out.txt | wc -l)
  [ $len == 1342 ]
}

@test "parse output as json" {
  cat out.txt | jq
}
