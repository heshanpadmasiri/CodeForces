#include<iostream>

using namespace std;

void solve() {
    int length;
    scanf("%d", &length);
    int oddIndex = -1;
    bool skip = false;
    for (int i = 0; i < length ; i++) {
        int val;
        scanf("%d", &val);
        if (skip) {
            continue;
        }
        // printf("val: %d\n", val);
        if (val % 2 == 0) {
            printf("1\n%d\n", i+1);
            skip = true;
        }
        else if (oddIndex != -1) {
            printf("2\n%d %d\n", oddIndex, i+1);
            skip = true;
        }
        oddIndex = i + 1;
    }
    if (!skip) {
        printf("-1\n");
    }
}

int	main()
{
    int count;
    scanf("%d", &count);
    while (count--)
    {
        solve();
    }

    return 0;
}
