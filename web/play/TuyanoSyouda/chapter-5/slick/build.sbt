name := "sample"
organization := "com.example"

version := "0.1"

scalaVersion := "2.13.0"

lazy val root = (project in file(".")).enablePlugins(PlayScala)

libraryDependencies += "com.h2database" % "h2" % "1.4.197"
libraryDependencies += "com.typesafe.play" %% "play-slick" % "5.0.0-M6"
libraryDependencies += "com.typesafe.play" %% "play-slick-evolutions" % "5.0.0-M6"
libraryDependencies += guice
libraryDependencies += "org.scalatestplus.play" %% "scalatestplus-play" % "4.0.3" % Test