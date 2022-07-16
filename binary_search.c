#include <stdio.h>

int main() {
    int search_value;
    int data[] = {1,2,4,5,6,7,8,9,10};
    scanf("%d", &search_value);
    search_number(search_value, data);
}

int search_number(int niddele, int haystack[]) {
    int low_index = 0;
    int high_index = sizeof(haystack) / sizeof(*haystack);
    int mid_index = low_index + high_index / 2;

    // while ()
    // {
    //     /* code */
    // }
    
}