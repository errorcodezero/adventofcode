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
    bool naughty = false;

    bool repeats = false;
    bool paired = false;

    auto pairs = std::unordered_map<std::string, int>();

    for (int i = 0; i < line.length(); i++) {
      std::string pair = line.substr(i, 2);
      std::string trio = line.substr(i, 3);

      if (trio[0] == trio[2])
        repeats = true;

      for (int j = i + 2; j < line.length() - 1; j++) {
        if (line.substr(j, 2) == pair) {
          paired = true;
        }
      }
    }

    naughty = !paired || !repeats;

    if (!naughty)
      nice++;

    std::cout << line << ": " << (naughty ? "NAUGHTY" : "NICE") << "\n";
  }

  std::cout << "Nice strings: " << nice << "\n";
  return 0;
}
