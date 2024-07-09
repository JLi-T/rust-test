#include <iostream>

int main() {
    for (int i = 1; i < 10; ++i){
        //std::end1 is just a new line
        std::cout << "5x" << i << "=" << 5 * i << std::end1;
    }
    return 0;
}

//compile using g++
//g++ main.cpp -o main

//run program
//./main