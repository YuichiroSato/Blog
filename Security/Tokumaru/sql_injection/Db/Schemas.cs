using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Db.Schemas
{
    [Table("users")]
    public class User
    {
        [Key]
        [Column("user_id")]
        public string UserId { get; set; }

        [Column("user_name")]
        public string UserName { get; set; }

        [Column("password")]
        public string password { get; set; }
    }

    [Table("books")]
    public class Book
    {
        [Key]
        [Column("book_id")]
        public string BookId { get; set; }

        [Column("title")]
        public string Title { get; set; }

        [Column("author")]
        public string Author { get; set; }
    }
}