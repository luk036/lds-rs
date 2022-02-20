#include <initializer_list>
#include <array>
#include <iostream>

int main() {
  auto a = std::array{1, 3, 4};
  auto [b, c, d] = a;
  std::cout << b << c << d << '\n';
}
