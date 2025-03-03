#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int main() {
  std::ifstream file;

  file.open("input.txt");

  if (!file.is_open()) {
    return 1;
  }

  std::string line;
  int nice = 0;
  while (std::getline(file, line)) {
    bool naughty = true;

    char prev = ' ';
    char prevprev = ' ';
    bool repeats = false;
    bool arePairs = false;
    std::string pairKey = "";
    std::unordered_map<std::string, int> pairs =
        std::unordered_map<std::string, int>();

    int i = 0;

    for (int i = 0; i < line.length(); i++) {
      char c = line[i];
      if (i >= 1) {
        prev = line[i - 1];
        pairKey = "";
        pairKey += prev;
        pairKey += c;
        if (prev != c)
          pairs[pairKey]++;
      }
      if (i >= 2)
        prevprev = line[i - 1];
      if (prevprev == c) {
        repeats = true;
      }
      if (pairs[pairKey] >= 2) {
        arePairs = true;
      }
    }

    naughty = arePairs && repeats;

    if (!naughty)
      nice++;

    std::cout << line << ": " << (naughty ? "NAUGHTY" : "NICE") << "\n";
  }

  std::cout << "Nice strings: " << nice << "\n";

  return 0;
}
