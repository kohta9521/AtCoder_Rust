import sys
H, A = map(int, input().split())
x = H / A 
if H / A == 0:
    print(x)
else:
    print(x + 1)