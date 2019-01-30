using System;
using System.Collections.Generic;
using Db.Schemas;

namespace sql_injection.Models
{
    public class BookListModel
    {
        public IEnumerable<Book> BookList { get; set; }
    }
}