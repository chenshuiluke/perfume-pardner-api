using Microsoft.AspNetCore.Mvc;
using perfume_pardner_api.Models;
namespace perfume_pardner_api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class FragranceController : ControllerBase
{

    private readonly ILogger<FragranceController> _logger;

    public FragranceController(ILogger<FragranceController> logger)
    {
        _logger = logger;
    }

    [HttpGet(Name = "GetAllFragrances")]
    public IEnumerable<FragranceDto> Get()
    {
        return Enumerable.Range(1, 5).Select(index => new FragranceDto("Aventus", "Creed", 123, ".", new Dictionary<string, List<string>>() {

        }, "1234" ))
        .ToArray();
    }
}
