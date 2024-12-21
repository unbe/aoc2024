import sys
import re
from functools import reduce
from operator import mul

with open(sys.argv[1], 'r') as f:
  lines = list(map(lambda l: list(l.strip()), f.readlines()))
  eline = lines.index([])
  mp = lines[:eline]
  moves = ''.join(map(lambda l: ''.join(l), lines[eline+1:]))

print('\n'.join(map(lambda l: ''.join(l), mp)), moves)

r, = [r for r in map(lambda ic: (ic[0], ic[1].index('@') if '@' in ic[1] else -1), enumerate(mp)) if r[1] >= 0]
print(r)
mp[r[0]][r[1]] = '.'
print('\n'.join(map(lambda l: ''.join(l), mp)), r)

dirs={}
dirs['<'] = (0, -1)
dirs['>'] = (0, 1)
dirs['^'] = (-1, 0)
dirs['v'] = (1, 0)

for mv in moves:
  dr = dirs[mv]
  newpos = None
  box = r
  while True:
    box = (box[0] + dr[0], box[1] + dr[1])
    if mp[box[0]][box[1]] != 'O':
      break
    if newpos is None:
      newpos = box
  if mp[box[0]][box[1]] == '#':
    continue
  if mp[box[0]][box[1]] != '.':
    raise 'What??'
  if newpos is not None:
    mp[box[0]][box[1]] = 'O'
    mp[newpos[0]][newpos[1]] = '.'
    r = newpos
  else:
    r = box
print('\n'.join(map(lambda l: ''.join(l), mp)), r)
sum = 0
for i in range(len(mp)):
  for j in range(len(mp[i])):
    if mp[i][j] == 'O':
      sum += 100*i + j
print(sum)


