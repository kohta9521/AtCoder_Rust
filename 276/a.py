import sys

# 最初に配列の中に書く文字列を格納
# 文字列を格納した配列をひっくり返す
# ひっくり返した配列の後ろから最初にaが出てきたところで止まるif文を作成
#この時点で後ろから何番目かのaが判明
#この後は配列の長さを出し、後ろからの配列の長さを引くことでこのというの答えを算出する

# 文字列を読み込み配列の中に格納する作業
str = input()
list = list(str)

print(str)
print(list) #この時点で配列の中に各文字列を格納することに成功

#文字列を格納した配列listを逆向きに設定
list.reverse()
print(list)

#配列の中身を一つずつ取ってきてAかそうでないかを確認するfor文を作成
#最初にresult文を使用してそもそもaがない場合わけをする
if "a" not in list:
    print(-1)
    sys.exit()

#xに対して配列の要素数の数を入力させる
length = len(list)
print(length)

#lengthの長さから１ずつ数字を減らす変数iの構文を作成
while(length == 0):
    if list not in "a":
        length -= length
    else:
        print(length)

    
