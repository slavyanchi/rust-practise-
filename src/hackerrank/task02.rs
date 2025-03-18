#include <iostream>
#include <vector>
using namespace std;

vector<int> compareTriplets(vector<int> a, vector<int> b) {
    int aliceScore = 0;
    int bobScore = 0;

    for (int i = 0; i < 3; i++) {
        if (a[i] > b[i]) {
            aliceScore++;
        } else if (a[i] < b[i]) {
            bobScore++;
        }
    }
    vector<int> result(2);
    result[0] = aliceScore;
    result[1] = bobScore;
    return result;
}

int main() {
    vector<int> a(3), b(3);
    for (int i = 0; i < 3; i++) cin >> a[i];
    for (int i = 0; i < 3; i++) cin >> b[i];

    vector<int> ans = compareTriplets(a, b);
    cout << ans[0] << " " << ans[1] << endl;
    return 0;
}
