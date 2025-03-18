#include <iostream>
using namespace std;

void staircase(int n) {
    for(int i = 1; i <= n; i++) {
        // Кiлькiсть пропускiв (пробiлiв) = n - i
        for(int j = 0; j < n - i; j++) {
            cout << ' ';
        }
        // Потiм друкуємо i символiв '#'
        for(int j = 0; j < i; j++) {
            cout << '#';
        }
        cout << "\n";
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    staircase(n);

    return 0;
}
