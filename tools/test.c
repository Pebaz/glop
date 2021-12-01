#include <stdio.h>

int digit_to_utf8(int arg1)
{
    int ret = 48;
    while (arg1)
    {
        arg1 -= 1;
        ret += 1;
    }
    return ret;
}

void main()
{
    for (int i = 0; i < 10; i++)
    {
        printf("%c\n", digit_to_utf8(i));
    }
}