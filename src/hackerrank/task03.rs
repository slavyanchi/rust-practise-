#include <iostream>
#include <vector>
using namespace std;

// Function that returns the sum of the array's elements.
long aVeryBigSum(const vector<long>& arr) {
    long sum = 0;
    for (long x : arr) {
        sum += x;
    }
    return sum;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n; // number of elements

    vector<long> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }

    long result = aVeryBigSum(arr);
    cout << result << "\n";

    return 0;
}
