part1 () { egrep -o 'mul\(\d+,\d+\)' | sed 's/mul.//g; s/,/*/; s/)/+/; $ s/+//'  | tr '\n' ' ' | bc -l; }
echo 'part1: '
part1 < $1
echo 'part2: '
tr '\n' ' ' < $1 | sed "s/do()/\n/g"  | sed "s/don't().*//g" | part1
