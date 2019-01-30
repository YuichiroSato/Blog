using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Security.Claims;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.Cookies;
using Microsoft.AspNetCore.Mvc;
using xss.Models;

namespace xss.Controllers
{
    public class TrapController : Controller
    {
        [HttpGet]
        public IActionResult Index()
        {
            return View();
        }

        [HttpGet("/Result")]
        public IActionResult Result()
        {
            var model = new ResultModel
            {
                SessionId = Request.Query["sid"].FirstOrDefault(),
            };
            return View(model);
        }
    }
}
