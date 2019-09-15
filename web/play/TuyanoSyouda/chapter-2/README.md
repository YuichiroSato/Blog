# Play framework 読書メモ

# デバック起動

Intelljとsbtでデバック起動できる。

```
sbt -jvm-debug 9999
run
```

Intelljで Run > Edit Configurations から Remote を追加し、portに9999を設定する。

# エントリポイント

Prod Mode

[https://github.com/playframework/playframework/blob/fe3236ad4356a5f7c7844c4fbf1022abc6eb6193/transport/server/play-server/src/main/scala/play/core/server/ProdServerStart.scala]
ProdServerStartのmain関数

Dev Mode

[https://github.com/playframework/playframework/blob/fe3236ad4356a5f7c7844c4fbf1022abc6eb6193/transport/server/play-server/src/main/scala/play/core/server/DevServerStart.scala]
DevServerStartのエントリポイントが状況によって呼び分けられる。
呼び分けているのはsbt pluginの startDevMode関数。
[https://github.com/playframework/playframework/blob/fe3236ad4356a5f7c7844c4fbf1022abc6eb6193/transport/server/play-server/src/main/scala/play/core/server/DevServerStart.scala]
