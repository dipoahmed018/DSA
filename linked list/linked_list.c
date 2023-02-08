#include <stdio.h>

struct link
{
    struct link *previous_item ;
    struct link *next_item;
    int item;
} ;


int main () {
    struct link object1;
    struct link object2;
    object1.item = 12;
    object2.item = 25;
    object1.next_item = &object2;
    object2.previous_item = &object1;
    object1.previous_item = NULL;
    printf("%d", (1^0)%2);
}