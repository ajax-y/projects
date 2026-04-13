#include <iostream>
using namespace std;

int main() {
    int num1, num2, num3;
    cout << "Enter num 1 : ";
    cin >> num1;
    cout << "Enter num 2 : ";
    cin >> num2;
    cout << "Enter num 3 : ";
    cin >> num3;
    
    if (num1 > num2){
        if (num1 > num3){
            cout << num1;
        }
    }
    else if (num2 > num3){
        cout << num2;
    }
    else {
        cout << num3;
    }
    return 0;
}
