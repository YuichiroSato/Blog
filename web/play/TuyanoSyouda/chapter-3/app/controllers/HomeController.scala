package controllers

import akka.util.ByteString
import javax.inject._
import play.api._
import play.api.mvc._
import play.api.http._

@Singleton
class HomeController @Inject()(cc: ControllerComponents) extends AbstractController(cc) {

  def index() = Action {
    Ok(views.html.index())
  }

  def index2() = Action {
    Ok(views.html.index2("YAMADA-Taro"))
  }

  def index3() = Action {
    Ok(views.html.index3(123, "YAMADA-Taro", "flower"))
  }

  def index4() = Action {
    Ok(views.html.index4())
  }

  def index5() = Action {
    Ok(views.html.index5())
  }

  def index6() = Action {
    Ok(views.html.index6("これはコンコローラーで用意したメッセージです。"))
  }

  def index7(p: Option[Int]) = Action {
    Ok(views.html.index7(
      "これはコンコローラーで用意したメッセージです。",
      p.getOrElse(0)))
  }

  def index8() = Action {
    val arr = List(
      "Yamada Taro",
      "Tanaka Hanako",
      "Ogawa Sachiko"
    )
    Ok(views.html.index8(
      "これはコンコローラーで用意したメッセージです。",
      arr))
  }

  def index9() = Action {
    val arr = List(
      "Yamada Taro",
      "Tanaka Hanako",
      "Ogawa Sachiko"
    )
    Ok(views.html.index9(
      "これはコンコローラーで用意したメッセージです。",
      arr))
  }

  def index10() = Action {
    val arr = List(
      "Yamada Taro",
      "Tanaka Hanako",
      "Ogawa Sachiko"
    )
    Ok(views.html.index10(
      "これはコンコローラーで用意したメッセージです。",
      arr))
  }

  def index11() = Action {
    val arr = List(
      List("Taro", "taro@yamada", "999-999"),
      List("Hanako", "hanako@flower", "888-888"),
      List("Sachiko", "sachiko@happy", "777-777")
    )
    Ok(views.html.index11(
      "これはコンコローラーで用意したメッセージです。",
      arr,
      List("Name", "Mail", "Tel")
    ))
  }

  def form() = Action {
    Ok(views.html.form("これはコンコローラーで用意したメッセージです。"))
  }

  def form2() = Action { request =>
    val form = request.body.asFormUrlEncoded
    val param = form.getOrElse(Map())
    val name = param.get("name").get(0)
    val passwrod = param.get("pass").get(0)

    Ok(views.html.form(s"name: $name, password: $passwrod"))
  }
}
