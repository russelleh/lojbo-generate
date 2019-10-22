#!/usr/bin/env bats

@test "root word file exist" {
  find ./data/gismu.txt
}

@test "filter root words" {
  len=$(cat out.json | wc -l)
  [ $len == 1342 ]
}

@test "parse output as json" {
  cat out.json | jq
}

@test "parse affix" {
  len=$(cat out.json | jq .affixes[] | wc -l)
  [ $len == 1433 ]
}
