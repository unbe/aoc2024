import sys
from collections import Counter

with open(sys.argv[1], 'r') as f:
  lines = f.readlines()
inp = [[int(x) for x in line.strip().split('   ')] for line in lines]
srt = [sorted(x) for x in zip(*inp)]
print('part1:', sum([abs(a-b) for a,b in zip(*srt)]))
cnt=Counter(srt[1])
print('part2:', sum([cnt.get(x,0)*x for x in srt[0]]))
