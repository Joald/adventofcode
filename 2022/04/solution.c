#include <stdio.h>

int part1() {
    FILE* f = fopen("input.txt", "r");
    char* line;
    size_t _unused;
    int count = 0;
    int lines = 0;
    while (getline(&line, &_unused, f) > 1) {
        lines++;
        int l1, r1, l2, r2;
        sscanf(line, "%d-%d,%d-%d", &l1, &r1, &l2, &r2);
        if (l2 <= l1 && l1 <= r2 && l2 <= r1 && r1 <= r2 
         || l1 <= l2 && l2 <= r1 && l1 <= r2 && r2 <= r1) {
            count++;
        }
    }
    printf("%d\n", count);
}


int main() {
    FILE* f = fopen("input.txt", "r");
    char* line;
    size_t _unused;
    int count = 0;
    int lines = 0;
    while (getline(&line, &_unused, f) > 1) {
        lines++;
        int l1, r1, l2, r2;
        sscanf(line, "%d-%d,%d-%d", &l1, &r1, &l2, &r2);
        if (l2 <= l1 && l1 <= r2 || l2 <= r1 && r1 <= r2 
         || l1 <= l2 && l2 <= r1 || l1 <= r2 && r2 <= r1) {
            count++;
        }
    }
    printf("%d\n", count);
}
