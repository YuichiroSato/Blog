package controllers

import akka.util.ByteString
import javax.inject._
import play.api._
import play.api.mvc._
import play.api.http._

@Singleton
class HomeController @Inject()(cc: ControllerComponents) extends AbstractController(cc) {

  def index() = Action {
    Ok("Welcome to Play Framework !")
  }

  def todo() = TODO

  def result() = Action {
    Result(
      header = ResponseHeader(200, Map.empty),
      body = HttpEntity.Strict(
        ByteString("This is sample text,"),
        Some("text/plain")
      )
    )
  }

  def html() = Action {
    Ok("<h1>Hello!</h1><p>This is sample message.</p>")
      .as("text/html")
  }

  def xml() = Action {
    Ok("<root><title>Hello!</title><message>This is sample message.</message></root>")
      .as("application/xml")
  }

  def json() = Action {
    Ok("{title:'Hello!', message:'This is sample message,'}")
      .as("application/json")
  }

  def withHeader() = Action {
    Ok("<h1>Hello!</h1><p>This is sample message.</p>")
      .withHeaders(
        ACCEPT_CHARSET -> "utf-8",
        ACCEPT_LANGUAGE -> "ja-JP"
      )
      .as("text/html")
  }

  def id(id: Int) = Action {
    Ok(s"<h1>Hello!</h1><p>ID = $id</p>")
      .as("text/html")
  }

  def name(id: Int, name: String) = Action {
    Ok(s"<h1>Hello!</h1><p>ID = $id, name = $name</p>")
      .as("text/html")
  }

  def option(id: Int, name: Option[String]) = Action {
    Ok(s"<h1>Hello!</h1><p>ID = $id, name = ${name.getOrElse("no-name")}</p>")
      .as("text/html")
  }

  def setCookie(name: Option[String]) = Action { request =>
    (name match {
      case Some(str) =>
        val cookie = request.cookies.get("name")
        Ok("<p>name send.</p>" +
          s"<p>cookie: ${cookie.getOrElse(Cookie("name", "no-name")).value}")
          .withCookies(Cookie("name", str))
          .bakeCookies()
      case None => Ok("<p>no name.</p>")
    }).as("text/html")
  }

  def setSession(name: Option[String]) = Action { request =>
    (name match {
      case Some(str) =>
        val session = request.session.get("name")
        Ok("<p>name send.</p>" +
          s"<p>session: ${session.getOrElse("no-name")}")
          .withSession(request.session + ("name" -> str))
      case None => Ok("<p>no name.</p>")
    }).as("text/html")
  }

}
