#include "commands.h"
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
  std::ifstream file;

  file.open("input.txt");

  Commands c = Commands(Commands::OFF);

  auto grid = std::vector<std::vector<bool>>();

  if (!file.is_open()) {
    return 1;
  }

  std::string line = "";

  while (std::getline(file, line)) {
  }

  return 0;
}
