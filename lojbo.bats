#!/usr/bin/env bats

@test "root word file exist" {
  find ./data/gismu.txt
}

@test "filter root words" {
  len=$(cargo run | wc -l)
  [ $len == 1342 ]
}
