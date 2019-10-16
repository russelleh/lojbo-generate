#!/bin/bash

mkdir -p data/

if [ ! -f data/gismu.txt ]; then
  curl https://mw.lojban.org/images/d/d8/gismu.txt \
    --output data/gismu.txt
fi
