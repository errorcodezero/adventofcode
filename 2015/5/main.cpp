#include <fstream>
#include <iostream>

int main() {
  std::ifstream file;

  file.open("input.txt");

  if (!file.is_open()) {
    return 1;
  }

  std::string line;
  while (std::getline(file, line)) {
    std::cout << line << std::endl;
  }

  return 0;
}
