using perfume_pardner_api.Core.Entities;

namespace perfume_pardner_api.Core.Interfaces
{
    public interface IFragranceRepository 
    {
        Task<IEnumerable<Fragrance>> GetAllAsync();
    }
}