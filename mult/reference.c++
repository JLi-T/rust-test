#include <iostream>

void increment(int& x) {
    x++;
}

int main() {
    int a = 5;
    increment(a);
    std::cout << a << std::endl; // Outputs: 6
    return 0;
}

// ignore the errors. This is just to compare

void increment(int x) {
    x++;
}

int main() {
    int a = 5;
    increment(a);
    std::cout << a << std::endl; // Outputs: 5
    return 0;
}
