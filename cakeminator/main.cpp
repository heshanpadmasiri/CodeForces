#include<iostream>
using namespace std;

int	main()
{
    int x, y;
    scanf("%d %d", &y, &x);
    bitset<11> row;
    bitset<11> column;
    for (int j = 0; j < y; j++)
    {
        char line[11];
        scanf("%s", line);
        for (int i = 0; i < x; i++)
        {
            if(line[i] == 'S') {
                row[i] = 1;
                column[j] = 1;
            }
        }
    }
    int count = 0;
    for (int j = 0; j < y; j++)
    {
        for (int i = 0; i < x; i++)
        {
            if (row[i] == 0 || column[j] == 0) {
                count++;
            }
        }
    }
    printf("%d\n", count);
    return 0;
}
