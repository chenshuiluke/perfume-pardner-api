using Microsoft.EntityFrameworkCore;
using perfume_pardner_api.Core.Entities;
using perfume_pardner_api.Core.Interfaces;
using perfume_pardner_api.Infrastructure.Data;

namespace perfume_pardner_api.Infrastructure.Repositories
{
    public class FragranceRepository : IFragranceRepository
    {
        private readonly ApplicationDbContext _context;

        public FragranceRepository(ApplicationDbContext context)
        {
            _context = context;
        }

        public async Task<IEnumerable<Fragrance>> GetAllAsync()
        {
            return await _context.Fragrances.ToListAsync();
        }

    }
}