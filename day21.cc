#include <fstream>
#include <iostream>
#include <memory>
#include <algorithm>
#include <numeric>
#include <map>
#include <vector>

typedef struct {
  int r, c;
} pos;

std::map<char, pos> remap(std::vector<std::string> kbd) {
  std::map<char, pos> k;
  for(int row = 0; row < kbd.size(); row++) {
    for(int col = 0; col < kbd[row].size(); col++) {
      auto key = kbd[row][col];
      pos here = {row, col};
      k[key] = here;
    }
  }
  return k;
}

typedef std::map<std::pair<char, char>, long> costmap;

costmap trivial(std::vector<std::string> kbd) {
  auto k = remap(kbd);
  costmap moves;
  for (const auto& kva : k) {
    for (const auto& kvb : k) {
      if (kva.first == '.' || kvb.first == '.') {
        continue;
      }
      moves[std::make_pair(kva.first, kvb.first)] = 1;
    }
  }
  return moves;
}

long move_cost(const std::string& move, costmap costs) {
  char at = 'A';
  long r = 0;
  for (const char p : move) {
    r += costs[std::make_pair(at, p)];
    at = p;
  }
  return r;
}

costmap solve(std::vector<std::string> kbd, costmap parent) {
   auto k = remap(kbd);
   pos bad = k['.'];
   costmap moves;
   for (const auto& kva : k) {
    if (kva.first == '.') {
      continue;
    }
    for (const auto& kvb : k) {
      if (kvb.first == '.') {
        continue;
      }
      if (kva.first == kvb.first) {
        moves[std::make_pair(kva.first, kvb.first)] = 1;
        continue;
      }
      auto a = kva.second;
      auto b = kvb.second;
      int ve = a.r - b.r;
      int hz = a.c - b.c;    
      auto leg_hz = std::string(abs(hz), hz > 0 ? '<' : '>');
      auto leg_ve = std::string(abs(ve), ve > 0 ? '^' : 'v');
      auto move_hzve = leg_hz + leg_ve + "A";
      auto move_vehz = leg_ve + leg_hz + "A";      
      auto cost_hz = move_cost(move_hzve, parent);
      auto cost_ve = move_cost(move_vehz, parent);
      bool no_vehz = a.c == bad.c && b.r == bad.r;
      bool no_hzve = a.r == bad.r && b.c == bad.c;
      auto cost = (no_vehz || (cost_ve > cost_hz && !no_hzve)) ? cost_hz : cost_ve;
      moves[std::make_pair(kva.first, kvb.first)] = cost;
    }
   }
   return moves;
}

int main(int argc, char *argv[])
{
  auto nextkbd = trivial({".^A","<v>"});
  int i = 0;
  for (; i < 2; i++) {
    nextkbd = solve({".^A","<v>"}, nextkbd);
  }
  auto p1costs = solve({"789","456","123",".0A"}, nextkbd);
  for (; i < 25; i++) {
    nextkbd = solve({".^A","<v>"}, nextkbd);
  }
  auto p2costs = solve({"789","456","123",".0A"}, nextkbd);

  std::ifstream inf(argv[1]);
  std::string line;
  long part1 = 0;
  long part2 = 0;
  while(std::getline(inf, line)) {
    const auto cost_p1 = move_cost(line, p1costs);
    const auto cost_p2 = move_cost(line, p2costs);
    std::string num_part;
    std::copy_if(line.begin(), line.end(), std::back_inserter(num_part), [] (char x) { return isdigit(x);});
    part1 += std::stoi(num_part) * cost_p1;
    part2 += std::stoi(num_part) * cost_p2;
  }
  std::cout << "part1: " << part1 << std::endl;  
  std::cout << "part2: " << part2 << std::endl;  
}