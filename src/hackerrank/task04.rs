#include <iostream>
#include <vector>
#include <cmath> // for abs()
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    // Зберiгаемо матрицю в 2D векторi або зчитуемо iнкрементно
    vector<vector<int>> arr(n, vector<int>(n));

    for(int i = 0; i < n; i++) {
        for(int j = 0; j < n; j++) {
            cin >> arr[i][j];
        }
    }

    long long primarySum = 0;
    long long secondarySum = 0;

    // Обчислюемо суму головноi дiагоналi i додатковоi
    // Головна дiагональ: arr[i][i]
    // Додаткова дiагональ: arr[i][n - 1 - i]
    for(int i = 0; i < n; i++) {
        primarySum += arr[i][i];
        secondarySum += arr[i][n - 1 - i];
    }

    long long result = llabs(primarySum - secondarySum);
    cout << result << "\n";

    return 0;
}
