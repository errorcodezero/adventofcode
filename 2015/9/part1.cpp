#include "node.h"
#include "utils.h"
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int main() {
  std::ifstream file;

  file.open("input.txt");

  std::string line = "";

  auto data = std::unordered_map<std::string, Node>();

  while (std::getline(file, line)) {
    // std::cout << line << "\n";
    std::vector<std::string> splitLine = split(line, " ");
    for (std::string line : splitLine) {
      std::cout << line << " ";
    }
    // CityA to CityB = Number
    // 0     1  2     3 4
    data[splitLine[0]].name = splitLine[0];
    data[splitLine[2]].name = splitLine[2];
    Connection conn = {.start = data[splitLine[0]],
                       .end = data[splitLine[2]],
                       .distance = std::stoi(splitLine[4])};
    data[splitLine[0]].connections.push_back(conn);
    data[splitLine[2]].connections.push_back(conn);
    std::cout << "\n";
    std::cout << std::stoi(splitLine[4]) << "\n";
    std::cout << data[splitLine[0]].toString() << "\n";
  }

  file.close();

  return 0;
}
