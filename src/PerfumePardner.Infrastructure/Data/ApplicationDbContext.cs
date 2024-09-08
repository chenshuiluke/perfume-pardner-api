using Microsoft.EntityFrameworkCore;
using perfume_pardner_api.Core.Entities;

namespace perfume_pardner_api.Infrastructure.Data
{
    public class ApplicationDbContext : DbContext
    {
        public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options) 
            : base(options)
        {
        }

        public DbSet<Fragrance> Fragrances { get; set; }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);
            // Add any additional configurations here
        }
    }
}