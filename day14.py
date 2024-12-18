import sys
import re
from functools import reduce
from operator import mul

with open(sys.argv[1], 'r') as f:
  p = []
  v = []
  for line in f.readlines():
    m = re.match(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)", line)
    p.append(list(map(int, m.groups()[0:2])))
    v.append(list(map(int, m.groups()[2:])))
  sz = (101, 103)
  mid = list(map(lambda x: (x-1)/2, sz))
  for nn in range(1, 9100):
    for i in range(len(p)):
      for si in range(len(sz)):
        p[i][si] = (p[i][si] + v[i][si]) % sz[si]
    sc = [0, 0, 0, 0]
    for pp in p:
      if  pp[0] != mid[0] and pp[1]!= mid[1]:
        sc[(pp[1] < mid[1]) + 2*(pp[0] <= mid[0])] += 1
    if nn == 100:
      print(reduce(mul, sc))
    if nn == 6668:
      pset = set(map(lambda pz: tuple(pz), p))
      for i in range(sz[1]):
        for j in range(sz[0]):
          if (j, i) in pset:
            print('*', end='')
          else:
            print('.', end='')
        print('')

