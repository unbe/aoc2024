#include <fstream>
#include <iostream>
#include <cstdlib>
#include <memory>
#include <vector>
#include <print>
#include <unordered_map>
#include <unordered_set>
#include <list>
#include <numeric>
#include <ranges>

typedef std::pair<int, int> rc;
auto rc_hash = [](const rc& v){
    return std::hash<int>()(v.first) * 100 + std::hash<int>()(v.second);
};

int main(int argc, char *argv[])
{
  std::ifstream inf(argv[1]);
  std::string line;
  std::vector<std::vector<int>> map;
  std::unordered_map<rc, std::unordered_set<rc, decltype(rc_hash)>, decltype(rc_hash)> front;
  while(inf >> line) {
    std::vector<int> row;
    std::transform(line.begin(), line.end(), std::back_inserter(row), [] (auto a) { return a - '0'; });
    for (int i = 0; i < row.size(); i++) {
      if (row[i] == 9) {
        rc nine = std::make_pair(map.size(), i);
        front[nine] = {nine};
      }
    }
    map.push_back(row);
    std::print("{}\n", row);
  }
  rc sz(map.size(), map[0].size());
  std::print("{}\n", front);
  rc dirs[] = {{-1,0}, {0, -1}, {1,0}, {0, 1}};
  for (int i = 0; i < 9; i++) {
    decltype(front) next_front;
    for (const auto& [f, nines] : front) {
      for(const auto& d : dirs) {
        rc nf = std::make_pair(f.first + d.first, f.second + d.second);
        if (nf.first >= 0 && nf.second >= 0 && 
            nf.first < sz.first && nf.second < sz.second && 
            map[nf.first][nf.second] + 1 == map[f.first][f.second]) {
          next_front[nf].insert(nines.begin(), nines.end());
        }
      }
    }
    front = next_front;
    std::print("{}\n", front);
  }
  auto scores = std::views::values(front) | std::views::transform([] (const auto& v) { return v.size(); }) | std::views::common;
  std::cout << "part1: " << std::accumulate(scores.begin(), scores.end(), 0) << std::endl;

}