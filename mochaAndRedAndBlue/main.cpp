#include<iostream>
#include<string>

using namespace std;

char other(char color) {
    if (color == 'B') {
        return 'R';
    }
    return 'B';
}

void solve() {
    int size;
    string row;
    cin >> size >> row;
    for (int i = 1; i < size; i++) {
        if (row[i] == '?' && row[i-1] != '?') {
            row[i] = other(row[i-1]);
        }
    }
    if (row[size-1] == '?') {
        row[size-1] = 'B';
    }
    for (int i = size - 2; i > -1; i--) {
        if (row[i] == '?') {
            row[i] = other(row[i+1]);
        }
    }
    cout << row << endl;
}

int	main()
{
    int count;
    cin >> count;
    while (count--)
    {
        solve();
    }

    return 0;
}
