using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations.Schema;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Db;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using Npgsql;
using sql_injection.Models;

namespace sql_injection.Controllers
{
    public class PlaceHolderController : Controller
    {
        private readonly InjectionDbContext db;

        public PlaceHolderController(InjectionDbContext db)
        {
            this.db = db;
        }

        public IActionResult Index()
        {
            var model = new BookListModel
            {
                BookList = db.Books.ToArray(),
            };
            return View(model);
        }

        [HttpPost]
        public IActionResult Post([FromForm] string key_word)
        {
            var str = $"%{key_word}%";
            var model = new BookListModel
            {
                BookList = db.Books.FromSql($"select * from Books where title like {str};").ToArray(),
            };
            return View("Index", model);
        }
    }
}
