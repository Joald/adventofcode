#include <iostream>
#include <sstream>
#include <fstream>
#include <array>

void part1() {
    int total_pts = 0;
    std::ifstream f("input.txt");
    int beats[3] = { 2, 0, 1 };
    for (std::string line; getline(f, line);) {
        std::istringstream stream(line);
        char theirs = line[0] - 'A';
        char yours = line[2] - 'X';
        total_pts += yours + 1;
        total_pts += beats[yours] == theirs ? 6 : yours == theirs ? 3 : 0;
    }
    std::cout << total_pts << "\n";
}

void part2() {
    int total_pts = 0;
    std::ifstream f("input.txt");
    int beats[3] = { 2, 0, 1 };
    int loses[3] = { 1, 2, 0 };
    int draws[3] = {0, 1, 2};
    for (std::string line; getline(f, line);) {
        std::istringstream stream(line);
        char theirs = line[0] - 'A';
        int result = line[2] - 'X';
        // 0 -> loss, 2 -> win
        int yours = result == 2 ? loses[theirs] : result == 1 ? theirs : beats[theirs];
        total_pts += yours + 1;
        total_pts += beats[yours] == theirs ? 6 : yours == theirs ? 3 : 0;
    }
    std::cout << total_pts << "\n";
}
int main() {
    part2();
}
