egrep -o 'mul\(\d+,\d+\)' $1 | sed 's/mul.//g; s/,/*/; s/)/+/; $ s/+//'  | tr '\n' ' ' | bc -l
cat $1 | tr '\n' ' ' | sed "s/do()/\n/g"  | sed "s/don't().*//g" | egrep -o 'mul\(\d+,\d+\)' | sed 's/mul.//g; s/,/*/; s/)/+/; $ s/+//'  | tr '\n' ' ' | bc -l
