//multiplication C++
// #include <iostream>

// int main() {
//     for (int i = 1; i < 10; ++i){
//         //std::end1 is just a new line
//         std::cout << "5x" << i << "=" << 5 * i << std::end1;
//     }
//     return 0;
// }

//compile using g++
//g++ main.cpp -o main

//run program
//./main

////////////////////////////////////////////////////////////////////////////

//iostream for std::out and in
#include <iostream>
#include <string>
#include <map>
#include <thread>
#include <chrono>

//& in c++ are references, unlike borrowing in rust
void add_item(std::map<std::string, int>& inv, const std::string& item_name, int quantity) {
    inv[item_name] += quantity;
    std::cout << "\nAdded " << quantity << " " << item_name << " to inventory" << std::endl;
}

void remove_item(std::map<std::string, int>& inv, const std::string& item_name, int quantity) {
    auto it = inv.find(item_name);
    if (it != inv.end()) {
        if (it->second >= quantity) {
            it->second -= quantity;
            std::cout << "\nRemoved " << quantity << " " << item_name << " from inventory" << std::endl;
            if (it->second == 0) {
                inv.erase(it);
                std::cout << "\nRemoved all of " << item_name << std::endl;
            }
        } else {
            std::cout << "\nNot enough " << item_name << " in inventory" << std::endl;
        }
    } else {
        std::cout << "\n" << item_name << " does not exist" << std::endl;
    }
}

void display_inv(const std::map<std::string, int>& inv) {
    std::cout << "Current collection: " << std::endl;
    for (const auto& item : inv) {
        std::cout << item.first << ": " << item.second << std::endl;
    }
    std::this_thread::sleep_for(std::chrono::seconds(1));
}

int main() {
    std::map<std::string, int> inv;

    while (true) {
        std::cout << "\n1. Add item to inventory" << std::endl;
        std::cout << "2. Remove item from inventory" << std::endl;
        std::cout << "3. Display inventory" << std::endl;
        std::cout << "4. Exit" << std::endl;
        std::cout << "Enter your choice (1-4): ";
        std::cout.flush();

        std::string choice;
        std::getline(std::cin, choice);

        if (choice == "1") {
            std::cout << "Enter item name: ";
            std::cout.flush();
            std::string item_name;
            std::getline(std::cin, item_name);

            std::cout << "Enter quantity: ";
            std::cout.flush();
            std::string quantity_str;
            std::getline(std::cin, quantity_str);
            int quantity = std::stoi(quantity_str);

            add_item(inv, item_name, quantity);
        } else if (choice == "2") {
            std::cout << "Enter item name: ";
            std::cout.flush();
            std::string item_name;
            std::getline(std::cin, item_name);

            std::cout << "Enter quantity: ";
            std::cout.flush();
            std::string quantity_str;
            std::getline(std::cin, quantity_str);
            int quantity = std::stoi(quantity_str);

            remove_item(inv, item_name, quantity);
        } else if (choice == "3") {
            display_inv(inv);
        } else if (choice == "4") {
            std::cout << "Exiting program..." << std::endl;
            break;
        } else {
            std::cout << "\nInvalid choice" << std::endl;
        }
    }

    return 0;
}
