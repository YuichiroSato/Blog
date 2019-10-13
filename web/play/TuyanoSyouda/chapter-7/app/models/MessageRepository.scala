package models

import java.util.Date
import java.sql.Timestamp
import javax.inject.{Inject, Singleton}
import play.api.db.slick.DatabaseConfigProvider
import slick.jdbc.JdbcProfile

import scala.concurrent.{ExecutionContext, Future}

@Singleton
class MessageRepository @Inject()
  (dbConfigProvider: DatabaseConfigProvider)
  (implicit ec: ExecutionContext) {

  private val dbConfig = dbConfigProvider.get[JdbcProfile]

  import dbConfig._
  import profile.api._

  private class PeopleTable(tag: Tag) extends Table[Person](tag, "people") {
    def id = column[Int]("id", O.PrimaryKey, O.AutoInc)
    def name = column[String]("name")
    def mail = column[String]("mail")
    def tel = column[String]("tel")

    def * = (id, name, mail, tel) <> ((Person.apply _).tupled, Person.unapply)
  }

  private val people = TableQuery[PeopleTable]

  private class MessagesTable(tag: Tag) extends Table[Message](tag, "messages") {
    def id = column[Int]("id", O.PrimaryKey, O.AutoInc)
    def personId = column[Int]("person_id")
    def message = column[String]("message")
    def created = column[Timestamp]("created")

    def * = (id, personId, message, created) <> ((Message.apply _).tupled, Message.unapply)
  }

  private val messages = TableQuery[MessagesTable]

  def listMsg: Future[Seq[Message]] = {
    db.run(messages.sortBy(_.created.desc).result)
  }

  def listMsgWithP(): Future[Seq[MessageWithPerson]] = {
    val query = messages
      .sortBy(_.id.desc)
      .join(people)
      .on(_.personId === _.id)
    db.run(query.result).map { obj =>
      obj.groupBy(_._1.id).toSeq
        .map { case (_, tuples) =>
          val (message, person) = tuples.head
          MessageWithPerson(message, person)
        }
        .sortBy(_.message.created.getTime)
        .reverse
    }
  }

  def getMsg(id: Int): Future[Message] = db.run {
    messages.filter(_.id === id).result.head
  }

  def createMsg(personId: Int, message: String): Future[Int] = db.run(
    messages += Message(0, personId, message, new Timestamp(new Date().getTime))
  )
}
