package controllers

import anorm._

object PersonForm {
  import play.api.data._
  import play.api.data.Forms._

  case class Data(name: String, mail: String, tel: String)
  case class PersonData(id: Int, name: String, mail: String, tel: String)

  val form = Form(
    mapping(
      "name" -> text,
      "mail" -> text,
      "tel" -> text
    )(Data.apply)(Data.unapply)
  )

  val personForm = Form(
    mapping(
      "id" -> number,
      "name" -> text,
      "mail" -> text,
      "tel" -> text
    )(PersonData.apply)(PersonData.unapply)
  )

  val personParser = {
    SqlParser.int("people.id") ~
      SqlParser.str("people.name") ~
      SqlParser.str("people.mail") ~
      SqlParser.str("people.tel")
  } map {
    case id ~ name ~ mail ~ tel => PersonData(id, name, mail, tel)
  }
}
