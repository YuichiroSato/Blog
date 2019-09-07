name := "sample"
organization := "com.example"

version := "0.1"

scalaVersion := "2.13.0"

lazy val root = (project in file(".")).enablePlugins(PlayScala)

libraryDependencies += guice
libraryDependencies += "org.scalatestplus.play" %% "scalatestplus-play" % "4.0.3" % Test