using Db.Schemas;
using Microsoft.EntityFrameworkCore;

namespace Db
{
    public class InjectionDbContext : DbContext
    {
        public InjectionDbContext(DbContextOptions options) : base(options) { }

        public DbSet<User> Users { get; set; }

        public DbSet<Book> Books { get; set; }
    }
}