#include <stdio.h>

typedef struct link
{
    struct link *previous_item ;
    struct link *next_item;
    int item;
} link;


int main () {
    
    link list[6];

    list[0].item = 1225;
    list[1].item = 33;
    list[0].next_item = &list[1];
    
    printf("%d", list[0].item);
}