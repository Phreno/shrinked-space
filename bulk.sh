#!/bin/bash
for i in {1..100}; do
value="0.`printf "%02d" $i`" 
  cargo run -- -s 256 -c "($value-$value)" -O s256c$i-$i.svg
done

