# Play framework 読書メモ

# play.api.dbコードリーディング

* [初期化](https://github.com/playframework/playframework/blob/ab6697f494a000ba8d19768e9d04eed3bb8cdbf7/persistence/play-jdbc/src/main/scala/play/api/db/DefaultDBApi.scala)
* [DefaultDatabase](https://github.com/playframework/playframework/blob/ab6697f494a000ba8d19768e9d04eed3bb8cdbf7/persistence/play-jdbc/src/main/scala/play/api/db/Databases.scala)
 PooledDatabase - デフォルトで作られるデータベースオブジェクト。HikariCPConnectionPoolをnewしている。
 DefaultDatabase - PooledDatabaseの親クラス。`withConnection` などが実装されている。
* [HikariCPModule](https://github.com/playframework/playframework/blob/ab6697f494a000ba8d19768e9d04eed3bb8cdbf7/persistence/play-jdbc/src/main/scala/play/api/db/HikariCPModule.scala)
 HikariCPConnectionPoolなどの実装がある。

# HikariCPコードリーディング

* HikariProxyConnection
 コントローラーの中で実際に使われているConnectionクラス。org.h2.jdbc.JdbcConnection (java.sql.Connectionの実装)に処理を委譲する。
* HikariProxyStatement
 コントローラーの中で実際に使われているStatementクラス。org.h2.jdbc.JdbcStatement (java.sql.Statementの実装)に処理を委譲する。

# H2コードリーディング

* [JdbcConnection](https://github.com/h2database/h2database/blob/version-1.4.197/h2/src/main/org/h2/jdbc/JdbcConnection.java)
 `createStatement` や `prepareStatement` の実装がある。
* [JdbcStatement](https://github.com/h2database/h2database/blob/version-1.4.197/h2/src/main/org/h2/jdbc/JdbcStatement.java)
 `executeQuery` や `executeUpdate` の実装がある。
