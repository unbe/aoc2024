#include <fstream>
#include <iostream>
#include <memory>
#include <regex>
#include <map>


typedef uint64_t cnt_t;

cnt_t make(std::map<std::string,cnt_t>& cache, const std::string& pattern, std::vector<std::string> towels) {
  if (pattern.size() == 0) {
    return 1;
  }
  auto cached = cache.find(pattern);
  if (cached != cache.end()) {
    return cached->second;
  }
  cnt_t ways = 0;
  for (const std::string& tw : towels) {
    if (pattern.find(tw) == 0) {
      ways += make(cache, pattern.substr(tw.length()), towels);
    }
  }
  cache[pattern] = ways;
  return ways;
}

void print(__int128 x) {
    if (x < 0) {
        putchar('-');
        x = -x;
    }
    if (x > 9) print(x / 10);
    putchar(x % 10 + '0');
}

int main(int argc, char *argv[])
{
  std::ifstream inf(argv[1]);
  std::string line;
  std::getline(inf, line);
  const std::regex ws_re(", ");
  std::vector<std::string> towels;
  std::copy(std::sregex_token_iterator(line.begin(), line.end(), ws_re, -1),
    std::sregex_token_iterator(), std::back_inserter(towels));
  std::getline(inf, line);
  int possible = 0;
  cnt_t ways = 0;
  while (std::getline(inf, line)) {
    std::map<std::string,cnt_t> cache;
    cnt_t w = make(cache, line, towels);
    if (w > 0) {
      possible++;
      ways += w;
    } 
  }
  std::cout << "part1: " << possible << std::endl;
  std::cout << "part2: " << ways << std::endl;
}