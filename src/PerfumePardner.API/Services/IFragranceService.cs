using perfume_pardner_api.Models;

namespace perfume_pardner_api.Services {

    public interface IFragranceService 
    {
       Task<IEnumerable<FragranceDto>> GetAllFragrancesAsync(); 
    }
}