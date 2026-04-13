num1 = int(input("Enter num 1 : "))
num2 = int(input("Enter num 2 : "))
num3 = int(input("Enter num 3 : "))

if (num1 > num2):
    if (num1 > num3):
        print(num1)
elif (num2 > num3):
    print(num2)
else:
    print(num3)
