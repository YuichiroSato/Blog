package controllers

import java.sql._
import javax.inject._
import play.api._
import play.api.mvc._
import play.api.http._
import play.api.db._
import PersonForm._
import anorm._

@Singleton
class HomeController @Inject()(db: Database, cc: MessagesControllerComponents) extends MessagesAbstractController(cc) {

  def index() = Action { implicit request =>
      db.withConnection { implicit conn =>
        val result = SQL("select * from people")
          .as(personParser.*)
        Ok(views.html.index("People Data.", result))
      }
  }

  def show(id: Int) = Action { implicit request =>
    db.withConnection { implicit conn =>
      val result = SQL("select * from people where id = {id}")
        .on("id" -> id)
        .as(personParser.single)
      Ok(views.html.show("People Data.", result))
    }
  }
  def add() = Action { implicit request =>
    Ok(views.html.add("フォームを記入して下さい。", form))
  }

  def create() = Action { implicit request =>
    val formdata = form.bindFromRequest
    val data = formdata.get
    db.withConnection { implicit conn =>
      SQL("insert into people values (default, {name}, {mail}, {tel})")
        .on(
          "name" -> data.name,
          "mail" -> data.mail,
          "tel" -> data.tel
        ).executeInsert()
    }
    Redirect(routes.HomeController.index)
  }

  def edit(id: Int) = Action { implicit request =>
    var formdata = personForm.bindFromRequest
    db.withConnection { implicit conn =>
      val data = SQL("select * from people where id = {id}")
        .on("id" -> id)
        .as(personParser.single)
        formdata = personForm.fill(data)
    }
    Ok(views.html.edit("フォームを編集して下さい。", formdata, id))
  }

  def update(id: Int) = Action { implicit request =>
    val formdata = form.bindFromRequest
    val data = formdata.get
    db.withConnection { implicit conn =>
      SQL("update people set name={name}, mail={mail}, tel={tel} where id = {id}")
        .on(
          "name" -> data.name,
          "mail" -> data.mail,
          "tel" -> data.tel,
          "id" -> id
        ).executeUpdate()
      Redirect(routes.HomeController.index)
    }
  }

  def delete(id: Int) = Action { implicit request =>
    db.withConnection { implicit conn =>
      val data = SQL("select * from people where id = {id}")
        .on("id" -> id)
        .as(personParser.single)
      Ok(views.html.delete("このレコーボを削除します。", data, id))
    }
  }

  def remove(id: Int) = Action { implicit request =>
    db.withConnection { implicit conn =>
      SQL("delete people where id = {id}")
        .on(
          "id" -> id
        ).executeUpdate()
      Redirect(routes.HomeController.index)
    }
  }
}
