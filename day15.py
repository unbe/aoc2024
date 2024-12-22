import sys
import re
import operator
from functools import reduce

def solve(mp, moves):
  dirs={}
  dirs['<'] = (0, -1)
  dirs['>'] = (0, 1)
  dirs['^'] = (-1, 0)
  dirs['v'] = (1, 0)

  pos, = [cr for cr in map(lambda ic: (ic[0], ic[1].index('@') if '@' in ic[1] else -1), enumerate(mp)) if cr[1] >= 0]
  mp[pos[0]][pos[1]] = '.'

  for mv in moves:
    dr = dirs[mv]
    front = [pos]
    shape = [front]
    blocked = False
    npos = None
    while len(front) > 0 and not blocked:
      nfront = set()
      for f in front:
        nf = (f[0] + dr[0], f[1] + dr[1])
        if mp[nf[0]][nf[1]] == '#':
          blocked = True
          break
        if npos is None:
          npos = nf
        if mp[nf[0]][nf[1]] == '.':
          continue
        nfront.add(nf)
        if dr[0] != 0 and mp[nf[0]][nf[1]] == '[':
          nfront.add((nf[0], nf[1]+1))
        if dr[0] != 0 and mp[nf[0]][nf[1]] == ']':
          nfront.add((nf[0], nf[1]-1))
      front = nfront
      shape.insert(0, front)
    if blocked:
      continue
    for front in shape:
      for s in front:
        mp[s[0] + dr[0]][s[1] + dr[1]] = mp[s[0]][s[1]]
        mp[s[0]][s[1]] = '.'
    pos = npos
  return sum([100*i + j if c in '[O' else 0 for i in range(len(mp)) for j, c in enumerate(mp[i])])


with open(sys.argv[1], 'r') as f:
  lines = list(map(lambda l: list(l.strip()), f.readlines()))
eline = lines.index([])
map1 = lines[:eline]
moves = ''.join(map(lambda l: ''.join(l), lines[eline+1:]))

repls = {}
repls['#'] = '##'
repls['O'] = '[]'
repls['.'] = '..'
repls['@'] = '@.'
map2 = list(map(lambda line: reduce(operator.add, [list(repls[x]) for x in line]), map1))

print('part1:', solve(map1, moves))
print('part2:', solve(map2, moves))