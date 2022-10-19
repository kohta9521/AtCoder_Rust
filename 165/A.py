#Kを入力
K = int(input())

#A,Bへの入力
A, B = map(int, input().split())

#フラグ
exist = False

#i = A, A + 1.......Bを調べる
for i in range(A, B + 1):
    #Kの倍数が見つかったらフラグをTrueにする
    if i % K == 0:
        exist = True

#出力
print("OK" if exist else "NG")