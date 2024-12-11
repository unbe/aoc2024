#include <fstream>
#include <iostream>
#include <cstdlib>
#include <memory>
#include <print>
#include <list>

typedef std::pair<int, int> id_sz;

long sum(const std::list<id_sz>& files) {
  long sum = 0;
  int pos = 0;
  for(const auto& f : files) {
    for(int i = 0; i < f.second; i++) {
      if (f.first > 0) {
        sum += f.first * pos;
      }
      pos++;
    }
  }
  return sum;
}

int main(int argc, char *argv[])
{

  std::ifstream inf(argv[1]);
  std::string inp;
  inf >> inp;
  std::list<id_sz> files;
  std::transform(inp.begin(), inp.end(), std::back_inserter(files), 
      [n = 0](auto a) mutable { return std::make_pair<>((n++ % 2 == 0 ? n / 2 : -1), a - '0'); });

  auto free_i = std::next(files.begin());
  auto file_i = std::prev(files.end());
  std::list<id_sz> part2_files = files;

  while (free_i != file_i) {
    int move = std::min(free_i->second, file_i->second);
    files.insert(std::next(free_i), std::make_pair<>(-1, free_i->second - move));
    free_i->first = file_i->first;
    free_i->second = move;
    files.insert(std::next(file_i), std::make_pair<>(-1, move));
    file_i->second -= move;
    do {
      free_i = std::next(free_i);
    } while(free_i != file_i && (free_i->second == 0 || free_i->first >= 0));
    while(free_i != file_i && (file_i->second == 0 || file_i->first < 0)) {
      file_i = std::prev(file_i);
    }
  }
  std::cout << "part1: " << sum(files) << std::endl;

  files = part2_files;
  for(file_i = std::prev(files.end()); file_i != files.begin(); file_i = std::prev(file_i)) {
    if (file_i->first < 0) {
      continue;
    }
    free_i = std::find_if(files.begin(), file_i, [&file_i] (const auto& s) {return s.first < 0 && s.second >= file_i->second;});
    if (free_i != file_i) {
      files.insert(std::next(free_i), std::make_pair<>(-1, free_i->second - file_i->second));
      free_i->first = file_i->first;
      free_i->second = file_i->second;
      file_i->first = -1;
    }
  }
  std::cout << "part2: " << sum(files) << std::endl;
}