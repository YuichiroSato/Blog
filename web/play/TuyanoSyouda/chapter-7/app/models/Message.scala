package models

import java.sql.Timestamp

import play.api.data.Form
import play.api.data.Forms._

object Message {
  val messageForm: Form[MessageForm] = Form {
    mapping(
      "personId" -> number,
      "message" -> nonEmptyText
    )(MessageForm.apply)(MessageForm.unapply)
  }
}

case class Message(id: Int, personId: Int, message: String, created: Timestamp)

case class MessageForm(personId: Int, message: String)

case class MessageWithPerson(message: Message, person: Person)
