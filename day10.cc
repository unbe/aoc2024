#include <fstream>
#include <iostream>
#include <memory>
#include <vector>
#include <set>
#include <numeric>
#include <ranges>

int main(int argc, char *argv[])
{
  std::ifstream inf(argv[1]);
  std::string line;
  std::vector<std::vector<int>> map;
  typedef std::pair<int, int> rc;
  std::vector<std::vector<rc>> front;
  while(inf >> line) {
    std::vector<int> row(std::from_range, line | std::views::transform([] (auto a) { return a - '0'; }));
    for (int i = 0; i < row.size(); i++) {
      if (row[i] == 9) {
        rc nine = std::make_pair(map.size(), i);
        front.push_back({nine});
      }
    }
    map.push_back(row);
  }
  rc sz(map.size(), map[0].size());
  rc dirs[] = {{-1,0}, {0, -1}, {1,0}, {0, 1}};
  for (int i = 0; i < 9; i++) {
    decltype(front) new_front;
    for (const auto& path : front) {
      const auto& loc = path.back();
      for(const auto& d : dirs) {
        rc nf = std::make_pair(loc.first + d.first, loc.second + d.second);
        if (nf.first >= 0 && nf.second >= 0 && 
            nf.first < sz.first && nf.second < sz.second && 
            map[nf.first][nf.second] + 1 == map[loc.first][loc.second]) {
          new_front.push_back(path);
          new_front.back().push_back(nf);
        }
      }
    }
    front = new_front;
  }
  auto count_me = front | std::views::transform([] (const auto& v) { return std::make_pair(v.front(), v.back()); });
  std::set counter(std::from_range, count_me);
  std::cout << "part1: " << counter.size() << std::endl;
  std::cout << "part2: " << front.size() << std::endl;
}