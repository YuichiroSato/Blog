package controllers

import javax.inject._
import play.api._
import play.api.mvc._
import play.api.http._
import play.api.data._
import play.api.data.Forms._

@Singleton
class FormController @Inject()(cc: MessagesControllerComponents) extends MessagesAbstractController(cc) {
  import MyForm._

  def index() = Action { implicit request =>
    Ok(views.html.form3("", myform))
  }

  def form() = Action { implicit request =>
    val form = myform.bindFromRequest
    val name = form.data.get("name")
    val pass = form.data.get("password")
    val radio = form.data.get("radio")
    Ok(views.html.form3(s"name: $name, pass: $pass, radio: $radio", myform))
  }
}
