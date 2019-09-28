# Play framework 読書メモ

# Twirlまとめ

* テンプレートはScalaのコードにコンパイルされる
* 変数の埋め込みは `__display__` 関数に渡されるコードになって、テンプレートに埋め込まれる
* レイアウトもテンプレートと同じようにコンパイルされる
* ifブロックはScalaのif文にコンパイルされる
* forブロックはScalaのfor文にコンパイルされる
* ユーザ定義のブロックはScalaの関数にコンパイルされる
* helperはライブラリに含まれる関数を呼び出すコードになる

# Twirlコードリーディング

* [Twirlプラグイン設定](https://github.com/playframework/twirl/blob/master/sbt-twirl/src/main/scala/play/twirl/sbt/SbtTwirl.scala)
* [パーサ](https://github.com/playframework/twirl/blob/master/parser/src/main/scala/play/twirl/parser/TwirlParser.scala)
* [コンパイラ](https://github.com/playframework/twirl/blob/fe20c77f0bb7737efc7d38cbcf56c57786604698/compiler/src/main/scala/play/twirl/compiler/TwirlCompiler.scala)

