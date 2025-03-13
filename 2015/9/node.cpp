#include "node.h"
#include <string>
#include <unordered_map>
#include <vector>

std::string Node::toString() {
  using namespace std::string_literals;

  std::string string = ""s;

  string += "{ name: \"";
  string += this->name;
  string += "\", connections: [";
  for (Connection connection : connections) {
    string += "{ start:  ";
    string += connection.start.get().name;
    string += ", end: ";
    string += connection.end.get().name;
    string += ", distance: ";
    string += std::to_string(connection.distance);
    string += " }, ";
  }
  string += "]";
  string += "}";

  return string;
}
