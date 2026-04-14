#include <iostream>
using namespace std;

int main() {
    int num, sum=0, i=1;
    cout << "Enter any number : ";
    cin >> num;
    while (i<=num) {
        sum+=i;
        i++;
    }
    cout << sum;
    return 0;
}
