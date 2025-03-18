#include <iostream>
#include <vector>
#include <climits> // for LLONG_MIN, LLONG_MAX
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    // Зчитуємо 5 елементів
    vector<long long> arr(5);
    for (int i = 0; i < 5; i++) {
        cin >> arr[i];
    }

    // Обчислюємо загальну суму
    long long total = 0;
    for (int i = 0; i < 5; i++) {
        total += arr[i];
    }

    // Знаходимо мінімальний і максимальний елементи
    long long mn = LLONG_MAX;
    long long mx = LLONG_MIN;
    for (int i = 0; i < 5; i++) {
        if (arr[i] < mn) {
            mn = arr[i];
        }
        if (arr[i] > mx) {
            mx = arr[i];
        }
    }

    // Мінімальна сума = total - (найбільший елемент)
    long long minSum = total - mx;
    // Максимальна сума = total - (найменший елемент)
    long long maxSum = total - mn;

    // Виводимо результат
    cout << minSum << " " << maxSum << "\n";

    return 0;
}
