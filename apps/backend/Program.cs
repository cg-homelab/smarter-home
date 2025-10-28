using System.Text.Json.Serialization;

using backend.Database;
using backend.Home;
using backend.User;
using backend.PowerMetric;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateSlimBuilder(args);

builder.Services.ConfigureHttpJsonOptions(options =>
{
    // options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonSerializerContext.Default);
});
builder.Services.AddDbContext<AppDbContext>(options =>
{
    options
      .UseNpgsql(
          connectionString:
            Environment.GetEnvironmentVariable("CONNECTION_STRING") ??
            throw new InvalidOperationException("CONNECTION_STRING environment variable is not set"))
      //.LogTo(Console.WriteLine)
      ;
});

builder.Services.AddScoped<IHomeService, HomeService>();
builder.Services.AddScoped<IUserService, UserService>();
builder.Services.AddScoped<IPowerMetricService, PowerMetricService>();

var app = builder.Build();

UserEndpoints.MapUserEndpoints(app);

app.Run();

