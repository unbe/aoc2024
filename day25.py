import sys
import re
from functools import reduce
from operator import mul

with open(sys.argv[1], 'r') as f:
  lastline = ""
  depth = None
  locks = []
  keys = []
  for line in f.readlines():
    line = line.strip()
    if lastline != "":
      for e in [i for i, x in enumerate([x != lastline[i] for i, x in enumerate(line)]) if x]:
        depth[e] = linenum - 1
    else:
      if depth is not None:
        dest.append(depth)
      
      if line[0] == '#':
        linenum, lineinc = 0, 1
        dest = locks
      else:
        linenum, lineinc = 7, -1
        dest = keys
      depth = [9] * len(line)
    lastline = line
    linenum += lineinc
  dest.append(depth)

  print('part1:', sum(all(x + y < 6 for x, y in zip(key, lock)) for key in keys for lock in locks))