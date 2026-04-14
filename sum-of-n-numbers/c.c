#include<stdio.h>

int main() {
    int num, sum = 0, i = 1;
    printf("Enter any number : ");
    scanf("%d",&num);
    while (i<=num) {
        sum+=i;
        i++;
    }
    printf("%d",sum);
}
