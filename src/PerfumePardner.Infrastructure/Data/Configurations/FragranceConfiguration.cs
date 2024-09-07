using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using perfume_pardner_api.Core.Entities;

namespace PerfumePardner.Infrastructure.Data.Configurations
{
    public class FragranceConfiguration : IEntityTypeConfiguration<Fragrance>
    {
        public void Configure(EntityTypeBuilder<Fragrance> builder)
        {
            builder.HasKey(f => f.Id);
            builder.Property(f => f.Title).IsRequired().HasMaxLength(200);
            builder.Property(f => f.Designer).IsRequired().HasMaxLength(100);
            builder.Property(f => f.Year).IsRequired();
            builder.Property(f => f.Thumbnail).HasMaxLength(500);
            builder.Property(f => f.ObjectId).IsRequired().HasMaxLength(50);
        }
    }
}