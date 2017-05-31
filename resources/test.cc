// test.cc
int main(int argc, char** argv) {
  // Comment in the function
  std::vector<std::string> data = {
      std::string("hello"),
      std::string("world"),
  };
  for (auto& str: data) {
    std::cout << str << std::endl;
  }
}
