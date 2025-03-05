#include "utils.h"
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
  std::ifstream file;

  file.open("input.txt");

  auto grid = std::vector<std::vector<int>>();

  for (int i = 0; i < 1000; i++) {
    grid.push_back(std::vector<int>());
    for (int j = 0; j < 1000; j++) {
      grid[i].push_back(0);
    }
  }

  if (!file.is_open()) {
    return 1;
  }

  std::string line = "";

  while (std::getline(file, line)) {
    Commands c = from_string(line);

    switch (c) {
    case Commands::ON:
      line = line.substr(8);
      break;
    case Commands::OFF:
      line = line.substr(9);
      break;
    case Commands::TOGGLE:
      line = line.substr(7);
      break;
    }

    std::vector<std::string> data = split(line, " through ");

    std::string startString = data.front().data();
    std::string endString = data.back().data();

    std::cout << startString << ",";
    std::cout << endString << "\n";

    std::vector<std::string> startStringVect = split(startString, ",");
    std::vector<std::string> endStringVect = split(endString, ",");

    auto start = std::vector<int>(
        {std::stoi(startStringVect[0]), std::stoi(startStringVect[1])});
    auto end = std::vector<int>(
        {std::stoi(endStringVect[0]), std::stoi(endStringVect[1])});

    for (int i = start.front(); i <= end.front(); i++) {
      for (int j = start.back(); j <= end.back(); j++) {
        switch (c) {
        case Commands::OFF:
          if (grid.at(i).at(j) > 0)
            grid.at(i).at(j)--;
          break;
        case Commands::ON:
          grid.at(i).at(j)++;
          break;
        case Commands::TOGGLE:
          grid.at(i).at(j) += 2;
          break;
        }
      }
    }
  }

  int lights = 0;

  for (std::vector<int> row : grid) {
    for (int light : row) {
      if (light)
        lights += light;
    }
  }

  std::cout << "Total Brightness: " << lights << "\n";

  return 0;
}
