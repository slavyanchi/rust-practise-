#include <iostream>
#include <vector>
#include <iomanip> // for setprecision
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    vector<int> arr(n);
    for(int i = 0; i < n; i++) {
        cin >> arr[i];
    }

    // Лічильники
    int positiveCount = 0;
    int negativeCount = 0;
    int zeroCount = 0;

    for(int i = 0; i < n; i++) {
        if(arr[i] > 0) {
            positiveCount++;
        } else if(arr[i] < 0) {
            negativeCount++;
        } else {
            zeroCount++;
        }
    }

    // Обчислення часток
    double posRatio = (double)positiveCount / n;
    double negRatio = (double)negativeCount / n;
    double zeroRatio = (double)zeroCount / n;

    // Вивід з 6 знаками після коми (за вимогою HackerRank)
    cout << fixed << setprecision(6);
    cout << posRatio << "\n" << negRatio << "\n" << zeroRatio << "\n";

    return 0;
}
