#include <stdio.h>

int sort(int items[], size_t length, char direction) {
    printf("%d %ld %s", items, length, direction);
}

int main() {
    int items[] = {1,4,2,0,10,3,4};
    size_t length = sizeof items / sizeof *items;
    sort(items, length, 'ASC');
}