# Cubing暗号の利用モード

## 概要

* 平文ブロック長：45byte
* 暗号文ブロック長：54byte
* 鍵長：可変
(編集中)

## 図

(編集中)

## 流れ

1. 平文をブロックに分ける
2. 必要があればパディングを用意する
3. 全てのブロックに対し、エンコードを行う
4. ブロックごとに暗号化を行う
5. ブロックをシャッフルする(「リプレイ攻撃対策」参照)

(編集中)

## パディング

平文ブロックが45byteに満たない場合、特定の文字を入れる。推奨はアスタリスク( * )。

## エンコード

本節では以下の記号を用いる。

$n$：現在操作しているブロックの番号(0,1,2,...)。
$C_i$：ブロックの$i$ Byte目の文字
$toas(C)$：特定の文字$C$のASCIIコードにおける番号
$fras(N)$：特定の数字$N$のASCIIコードにおける文字
$floor(F)$：実数$F$が整数になるよう切り捨てた値
$F$：$\{n\bmod (52*52)\}<26*52$ の時'A'、$\{n\bmod (52*52)\}\geqq 26*52$の時'a'
$S$：$\{n\bmod (52*52)\}<26$ の時'A'、$\{n\bmod (52*52)\}\geqq 26$の時'a'

この時、

```math
\begin{aligned}

C_{45+i(i=1,2,3,4,5)} &= fras(\{(\Sigma_{j=9*(i-1)+1}^{9*i}{toas(C_j)})\bmod26\}+97)\\

C_{51} &= fras(toas(F)+floor(n/26))\\

C_{52} &= fras(toas(S)+floor(n\bmod26))\\

\end{aligned}
```

である。

(編集中)

## リプレイ攻撃対策

(編集中)

### ブロックが$52^2$以上ある時の処理