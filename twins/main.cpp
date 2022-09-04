#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;

int	main()
{
    int n;
    cin >> n;
    vector<int> coins;
    // vector<long> restSum;
    coins.reserve(n);
    // restSum.reserve(n);
    for (int i=0; i < n; i++) {
        int val;
        cin >> val;
        coins.push_back(val);
        // restSum.push_back(-1);
    }

    sort(coins.begin(), coins.end(), greater<int>());
    long currentSum = 0;
    for (int i = 0; i < n; i++) {
        currentSum += coins[i];
        long restSum = 0;
        for (int j = i+1; j < n; j++) {
            restSum += coins[j];
        }
        if (currentSum > restSum) {
            cout << i + 1 << endl;
            return 0;
        }
    }
    cout << n << endl;
    return 0;
    // long currentSum = 0;
    // for(int i = n-1; i >= 0; i--) {
    //     currentSum += coins[i];
    //     restSum[i] = currentSum;
    // }
    // currentSum = 0;
    // for(int i = 0; i < n; i++) {
    //     if (i == n-1) {
    //         cout << n << endl;
    //         return 0;
    //     }
    //     currentSum += coins[i];
    //     long rest = restSum[i+1];
    //     if (currentSum > rest) {
    //         cout << i+1 << endl;
    //         return 0;
    //     }
    // }
}
