# 一般化線形モデル(GLM)の読書メモ

## ポアソン回帰の統計モデル
説明変数がxi、応答変数がyi。

p(yi|λi) = λi^yi exp(-λi) / yi!

## リンク関数
ポアソン分布の平均λが説明変数の関数になっている。

λi = exp(b1 + b2 xi)

パラメータb1とb2を最尤推定で求めれば、あるxにおけるyの分布が計算できるようになる。

## デモ

### ポアソン回帰の例

[poisson-regression.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-03/poisson-regression.R)ではポアソン回帰で得られる回帰曲線の例を見られるようにした。

ポアソン回帰では、リンク関数を通して、説明変数xに対してyがどのように分布するのかが得られる。
リンク関数は以下のようになる。
パラメータb1, b2を与えると、そのパラメータを使ったリンク関数を返す。

```
linkFunction <- function(b1, b2) {
  function(x) {
    exp(b1 + b2 * x)
  }
}
```

リンク関数があれば、あるxに対してyの分布が得られる。
つまり、yを引数に取りyが起きる確率を返す関数が得られる。
以下の関数により得られる関数でyの起きる確率が計算される。

```
distribution <- function(x, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    dpois(y, lambda = l(x))
  }
}
```

本当のポアソン回帰ではb1とb2は`glm`を使って計算するが、とりあえず例が見たいだけなので、自分で与えた様々な値についてグラフをプロットしてみることにした。
実行すると、いろいろな場合についてグラフがプロットされる。
実際の回帰では、このようないろいろな曲線から、データに合うものが選ばれるわけである。
具体例として、観ていて面白いのは`b1 = -0.1`、`b2 = 0.1`の場合である。
b2が正なのでxが大きくなるほどリンク関数の指数の肩は大きな数になる。
つまり、そのxではyは大きな平均を持つポアソン分布をする。
逆に小さなxではyは小さな平均を持つポアソン分布をする。
平たく言うと、xが大きくなるにしたがって、yも概ね大きくなるということである。
ただ大きくなると言うだけでなく、どれくらいの分布を持って大きくなるかも分かるのがミソである。

### サンプルデータのポアソン回帰

[glm.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-03/glm.R)では本の中で触れられているサンプルデータを使って、実際にポアソン回帰をしてみる。
サンプルデータは[ここ](http://hosho.ees.hokudai.ac.jp/~kubo/ce/IwanamiBook.html)で配布されていたので、[data3a.csv](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-03/data3a.csv)にコピーしておいた。

ポアソン回帰をやっているのは以下のコードである。

```
fit <- glm(y ~ x, data = d, family = poisson)

b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]
```

`b1`と`b2`が最尤推定されたリンク関数のパラメータである。
このパラメータを使ったリンク関数を使えば、説明変数`x`に対して、どのように`y`が分布するかが分かる。
どのように`y`が分布するかをグラフにプロットしてみた。
`x = 0, 5, 10, 15, 20`の場合についてプロットした。
グラフを見ると`x`が大きくなるにしたがって`y`の平均値が大きくなるのが分かる。
これは、推定されたパラメータの`b2`が正であるためである。
リンク関数は`x`の指数関数であるため、`x`に対して分布の平均は指数的に大きくなる。
グラフを見ると、確かに分布のピークが指数的にずれていっているのが分かる。

### ポアソン回帰の回帰曲線のピーク

[fitted.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-03/fitted.R)ではサンプルデータのポアソン回帰で得られた分布のピークの位置を出力してみた。
ポアソン回帰によって、説明変数`x`に対する`y`の分布が得られる。
得られる分布はポアソン分布なので、最大の確立を与えるピークが一つある。
それがどう動くかを、もっと具体的に見るために出力してみた。
コードを実行してみると、`x`が1から20までの場合について、どの辺に最大値がでるかざっくり出力している。