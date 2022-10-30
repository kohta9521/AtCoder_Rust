from http.client import OK
import sys
import random

#最初に渡される初期値を読み込む
# ストリング型の数字とイント型の数字の紐付け 紐付けはfor文を使用して組み合わせ
#組み合わせ完了後 組み合わせ内容の仕分け ３種類の仕分け作業 if文で行う
#箱に当てはめていく作業
#箱の中でX軸とY軸での最大の三個の数字の多さを判別
#10*10マスだから２０通りのif分を使用 iを使用したfor文で管理
#その最大値をX軸Y軸で算出
#算出後X,Y軸の最大値の方向に対して傾ける
#この作業を繰り返す つまりここまでの作業を一つの関数として行う


#初期値として渡される各キャンディーの味の情報の読み込み
#この部分はint型で読み込む
num = 0
while num <= 100:
    num += 1

x[num] = list(map(int, input()))

#ここで各キャンディーの味で仕分けするべくnumの値のif文を作成
#先にキャンディーの味を格納する配列を作成
s_empty = []
w_empty = []
p_empty = []
if num == 1:
    s_empty.append(x[num])
elif num == 2:
    w_empty.append(x[num])
else:
    p_empty.append(x[num])


#仮に左上から１〜１００までの数字を場所に振ってその場所にランダム関数を使用して入れていくとする
#その場合入った場所を100通り確定させる必要がある
#ランダム関数を使用して各マスにどの番号が入るのか確定させる作業を行う
random.randint(1,100)
num = 0
while num <= 100:
    num += 1

add = 0
while add <= 100:
    add += 1
random.choice(add)

#どこにも帰属しない繰り返し構文
sample = 0
while sample <= 100:
    sample +=1

#このデータにどこのますにランダムで何味のキャンディーが入るか決定
data = {num[sample]: add[sample]}

#各マスを確定 各行ごとの最大値を求める
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 20:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()
if num[sample] <= 10:
    if x[num] = 3:
        add()
    elif x[num] = 2:
        sec()
    else:
        sec()

    




