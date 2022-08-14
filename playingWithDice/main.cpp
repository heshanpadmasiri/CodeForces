#include<iostream>

using namespace std;

int	main()
{
    int a, b;
    scanf("%d %d", &a, &b);
    int a_win, b_win, draw;
    a_win = b_win = draw = 0;
    for (int i = 1; i < 7; i++)
    {
        int a_dist = abs(a - i);
        int b_dist = abs(b - i);
        if (a_dist < b_dist) {
            a_win++;
        }
        else if (b_dist < a_dist) {
            b_win++;
        }
        else {
            draw++;
        }
    }
    printf("%d %d %d\n", a_win, draw, b_win);
    return 0;
}
