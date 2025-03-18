#pragma once

#include <functional>
#include <memory>
#include <string>
#include <vector>
struct Node;

struct Connection {
  std::shared_ptr<Node> start;
  std::shared_ptr<Node> end;
  int distance = 0;
};

struct Node {
  std::vector<std::shared_ptr<Connection>> connections =
      std::vector<std::shared_ptr<Connection>>();
  std::string name = "";
  std::string toString();
};

int getShortestPath(std::shared_ptr<Node> start);
int factorial(int number);
