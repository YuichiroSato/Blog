package controllers

object MyForm {
  import play.api.data._
  import play.api.data.Forms._

  case class Data(name: String, pass: String, radio: String)

  val myform = Form(
    mapping(
      "name" -> text,
      "pass" -> text,
      "radio" -> text
    )(Data.apply)(Data.unapply)
  )
}
