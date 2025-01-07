#include <fstream>
#include <iostream>
#include <memory>
#include <unordered_map>
#include <unordered_set>

int main(int argc, char *argv[])
{
  std::ifstream inf(argv[1]);
  std::string line;
  std::vector<std::string> map;
  typedef struct {
    int r;
    int c;
  } pos;
  pos S, E;
  while(std::getline(inf, line)) {
      int sc = line.find('S');
      if (sc >= 0) {
        S = {int(map.size()), sc};
      }
      int ec = line.find('E');
      if (ec >= 0) {
        E = {int(map.size()), ec};
      }
      map.push_back(line);
  }
  std::vector<pos> todo = {E};
  std::vector<pos> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
  auto eq = [](pos a, pos b) { return a.r == b.r && a.c == b.c; };
  auto hash = [](pos a) { return int(a.r*353 + a.c*5531); };
  std::unordered_map<pos, int, decltype(hash), decltype(eq)> dist(100, hash, eq);
  pos sz{int(map.size()), int(map[0].size())};
  while(todo.size() > 0) {
    pos p = todo.back();
    todo.pop_back();
    for (const auto& d : dirs) {
      pos np = {p.r + d.r, p.c + d.c};
      if (np.r < 0 || np.r >= sz.r || np.c < 0 || np.c >= sz.c || map[np.r][np.c] == '#' || dist.find(np) != dist.end()) {
        continue;
      }
      dist[np] = dist[p] + 1;
      todo.push_back(np);
    }
  }

  auto solve = [&dist, &hash, &eq] (int maxcheat, int mingain) {
    std::unordered_set<pos, decltype(hash), decltype(eq)> cheat_set(10, hash, eq);
    for(int c = -maxcheat; c <= maxcheat; c++) {
      for(int r = -(maxcheat-abs(c)); r <= maxcheat-abs(c); r++) {
        cheat_set.insert({c, r});
      }
    }
    int cnt = 0;
    for(const auto& dd : dist) {
      for(const auto& cheat : cheat_set) {
        pos cheat_to{dd.first.r + cheat.r, dd.first.c + cheat.c};
        auto cheat_i = dist.find(cheat_to);
        if (cheat_i == dist.end()) {
          continue;
        }
        int gain = dd.second - cheat_i->second - (abs(cheat.r) + abs(cheat.c));
        if (gain >= mingain) {
          cnt++;
        }
      }
    }
    return cnt;
  };
  std::cout << "part1: " << solve(2, 100) << std::endl;
  std::cout << "part2: " << solve(20, 100) << std::endl;
}
