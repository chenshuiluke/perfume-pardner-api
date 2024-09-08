using Microsoft.AspNetCore.Mvc;
using perfume_pardner_api.Models;
using perfume_pardner_api.Services;
namespace perfume_pardner_api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class FragranceController : ControllerBase
{
    private readonly ILogger<FragranceController> _logger;
    private readonly IFragranceService _fragranceService;

    public FragranceController(ILogger<FragranceController> logger, IFragranceService fragranceService)
    {
        _logger = logger;
        _fragranceService = fragranceService;
    }

    [HttpGet(Name = "GetAllFragrances")]
    public async Task<ActionResult<IEnumerable<FragranceDto>>> GetAllFragrances()
    {
        var fragrances = await _fragranceService.GetAllFragrancesAsync();
        return Ok(fragrances);
    }
}
