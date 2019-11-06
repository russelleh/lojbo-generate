#!/bin/bash
rm -f dictionary.ts dictionary.html
echo -n "const words = " >> dictionary.ts
cat out.json | jq -scM   >> dictionary.ts
cat dictionary.ts.src    >> dictionary.ts
node dictionary.ts > dictionary.html
