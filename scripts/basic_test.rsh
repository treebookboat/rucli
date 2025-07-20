#!/usr/bin/env rucli
# ワンライナー形式
for i in 1 2 3; do echo "Outer loop: $i"; for j in a b c; do echo "  Inner loop: $i$j"; done; done