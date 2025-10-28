using backend.Database;
namespace backend.PowerMetric;

public interface IPowerMetricService
{
    Task Post(PowerMetricModel metric);
}

public class PowerMetricService : IPowerMetricService
{
    private readonly AppDbContext _dbContext;

    public PowerMetricService(Database.AppDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async Task Post(PowerMetricModel metric)
    {
        _dbContext.Add<PowerMetricModel>(metric);
        await _dbContext.SaveChangesAsync();
    }
}
