part1 () { egrep -o 'mul\(\d+,\d+\)' | sed 's/mul.//g; s/,/*/; s/)/+/; $ s/+//'  | tr '\n' ' ' | bc -l; }
echo 'part1: '
cat $1 | part1
echo 'part2: '
cat $1 | tr '\n' ' ' | sed "s/do()/\n/g"  | sed "s/don't().*//g" | part1
