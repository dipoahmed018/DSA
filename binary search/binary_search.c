#include <stdio.h>

/**
 * @brief Searching the index of a value inside an array
 *
 * @param niddele This the value that is searched for
 * @param haystack This is the araay that is being searched on
 * @return int
 */
int search_number(int niddele, int haystack[], size_t length)
{
    int low_index = 0;
    int high_index = length / sizeof(*haystack);
    while (low_index <= high_index)
    {
    int mid_index = low_index + high_index / 2;

        if (haystack[mid_index] == niddele)
        {
            return mid_index;
        }

        if (haystack[mid_index] < niddele)
        {
            low_index = mid_index + 1;
        }
        else
        {
            high_index = mid_index - 1;
        }
    }

    return -1;
}

int main()
{
    int search_value;
    int data[] = {1, 2, 4, 5, 6, 7, 8, 9, 10};
    int length = sizeof(data);
    printf("Type the value you want to search for ");
    scanf("%d", &search_value);
    int index_of_searched_value = search_number(search_value, data, length);
    if (index_of_searched_value >= 0)
    {
        printf("The index is %d \nThe value is %d\n", index_of_searched_value, data[index_of_searched_value]);
    }
    else
    {
        printf("The index was not found \n");
    }
    return 0;
}
