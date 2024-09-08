using AutoMapper;
using perfume_pardner_api.Models;
using perfume_pardner_api.Core.Entities;

namespace perfume_pardner_api.API.Helpers
{
    public class AutoMapperProfile : Profile
    {
        public AutoMapperProfile()
        {
            CreateMap<Fragrance, FragranceDto>().ReverseMap();
        }
    }
}