#pragma once

#include <functional>
#include <memory>
#include <string>
#include <vector>
struct Node;

struct Connection {
  std::reference_wrapper<Node> start;
  std::reference_wrapper<Node> end;
  int distance = 0;
};

struct Node {
  std::vector<std::reference_wrapper<Connection>> connections =
      std::vector<std::reference_wrapper<Connection>>();
  std::string name = "";
  std::string toString();
  // Assignment operator
  Node &operator=(Node &other) { return other; };
};

std::vector<std::vector<Node>>
getPermutations(std::unordered_map<std::string, Node> data);
