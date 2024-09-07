using Microsoft.OpenApi.Models;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddControllers();

// Configure Swagger/OpenAPI
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new OpenApiInfo { Title = "Perfume Pardner API", Version = "v1" });
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI(c => 
    {
        c.SwaggerEndpoint("/swagger/v1/swagger.json", "Perfume Pardner API v1");
        // Serve Swagger UI at the app's root
        c.RoutePrefix = string.Empty;
    });
}

// Comment out HTTPS redirection for Docker environment
// app.UseHttpsRedirection();

app.UseAuthorization();
app.MapControllers();

app.Run();