#include <vector>
#include <iostream>

using namespace std;

int solve() {
    int size;
    cin >> size;
    vector<int> arr;
    arr.reserve(size);
    for (int i=0; i<size; i++) {
        int tmp;
        cin >> tmp;
        arr.push_back(tmp);
    }
    int ans = 0;
    int largest = arr[0];
    for (int i=1; i<size; i++) {
        if (arr[i] > largest) {
            largest = arr[i];
        }
        else {
            ans += 1;
            largest = -1;
        }
    }
    return ans;
}

int	main()
{
    int count;
    cin >> count;
    while (count--)
    {
        int result = solve();
        cout << result << endl;
    }

    return 0;
}
