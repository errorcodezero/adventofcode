#include "node.h"
#include <memory>
#include <string>
#include <set>

std::string Node::toString() {
  using namespace std::string_literals;

  std::string string = ""s;

  string += "{ name: \"";
  string += this->name;
  string += "\", connections: [";
  for (auto connection : connections) {
    string += "{ start:  ";
    string += connection->start->name;
    string += ", end: ";
    string += connection->end->name;
    string += ", distance: ";
    string += std::to_string(connection->distance);
    string += " }, ";
  }
  string += "]";
  string += "}";

  return string;
}

int factorial(int number) {
  if (number == 1) {
    return 1;
  } else {
    return number * factorial(number - 1);
  }
}

/*
* Brute forces all the routes and only 
* after doing all that it returns a value. 
* Not the best solution but I couldn't find anything elegant.
*/
int getShortestPath(std::shared_ptr<Node> start, int cities) {
  auto allRoutes = std::set<std::set<std::shared_ptr<Node>>>();
  auto visited = std::set<std::shared_ptr<Node>>();
  int length = 0;
  
  for (int i = 0; i < factorial(cities); i++) {
    for (int j = 0; j < cities; j++) {
    for (std::shared_ptr<Connection> connection : start->connections) {
      if (visited.contains)
    }

    allRoutes.insert(visited);
  }
}
