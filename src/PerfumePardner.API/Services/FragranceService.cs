using AutoMapper;
using perfume_pardner_api.Core.Interfaces;
using perfume_pardner_api.Models;

namespace perfume_pardner_api.Services 
{
    public class FragranceService : IFragranceService 
    {
        private readonly IFragranceRepository _fragranceRepository;
        private readonly IMapper _mapper;
        public FragranceService(IFragranceRepository fragranceRepository, IMapper mapper)
        {
            _fragranceRepository = fragranceRepository;
            _mapper = mapper;
        }
       public async Task<IEnumerable<FragranceDto>> GetAllFragrancesAsync() 
        {
            var fragrances = await _fragranceRepository.GetAllAsync();
            return _mapper.Map<IEnumerable<FragranceDto>>(fragrances);
        }
    }
}