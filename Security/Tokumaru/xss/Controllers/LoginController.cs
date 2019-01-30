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
    public class LoginController : Controller
    {
        [HttpGet]
        public async void Index()
        {
            // Login
            var claims = new List<Claim>
            {
                new Claim(ClaimsIdentity.DefaultNameClaimType, "username"),
            };
            var identity = new ClaimsIdentity(claims, CookieAuthenticationDefaults.AuthenticationScheme);
            var principal = new ClaimsPrincipal(identity);
            await HttpContext.SignInAsync(CookieAuthenticationDefaults.AuthenticationScheme, principal);

            var keyword = Request.Query["keyword"].FirstOrDefault();
            var str = $"<body>Keyword: {keyword}</body>";
            var byteArray = str.Select(c => (byte)c).ToArray();
            await this.Response.Body.WriteAsync(byteArray);
        }
    }
}
