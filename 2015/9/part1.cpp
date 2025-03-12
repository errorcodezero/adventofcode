#include "utils.h"
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
  std::ifstream file;

  file.open("input.txt");

  std::string line = "";

  while (std::getline(file, line)) {
    std::vector<std::string> split_line = split(line, " ");
    for (std::string_view c : split_line) {
      std::cout << c << " ";
    }
    std::cout << "\n";
  }

  return 0;
}
