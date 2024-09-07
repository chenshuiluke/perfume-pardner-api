using Microsoft.AspNetCore.Mvc;

namespace perfume_pardner_api.Controllers;

[ApiController]
[Route("[controller]")]
public class FragranceController : ControllerBase
{
    private static readonly string[] Summaries = new[]
    {
        "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
    };

    private readonly ILogger<FragranceController> _logger;

    public FragranceController(ILogger<FragranceController> logger)
    {
        _logger = logger;
    }

    [HttpGet(Name = "GetAllFragrances")]
    public IEnumerable<Fragrance> Get()
    {
        return Enumerable.Range(1, 5).Select(index => new Fragrance("Aventus", "Creed", 123, ".", new Dictionary<string, List<string>>() {

        }, "1234" ))
        .ToArray();
    }
}
