#include<iostream>

using namespace std;

int	main()
{
    int count;
    scanf("%d", &count);
    while (count--)
    {
        int l, r, k;
        scanf("%d %d %d", &l, &r, &k);
        if (l == r) {
            if (l == 1) {
                printf("NO\n");
            }
            else {
                printf("YES\n");
            }
            continue;
        }
        // what we need here is integer division
        int oddCount = (r - l + 1) - ((r/2) - ((l-1)/2));
        if (oddCount <= k) {
            printf("YES\n");
        }
        else {
            printf("NO\n");
        }
    }

    return 0;
}
