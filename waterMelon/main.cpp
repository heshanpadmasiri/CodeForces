#include<bits/stdc++.h>

using namespace std;

int	main()
{
    int weight;
    scanf("%d", &weight);
    int x = 2;
    while (x <= 100)
    {
        int y = x;
        while (y > 0) {
            int total = x + y;
            if (total == weight)
            {
                printf("YES\n");
                return 0;
            }
            
            y -= 2;
        }
        x += 2;
    }
    
    printf("NO\n");
    return 0;
}
