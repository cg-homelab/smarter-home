namespace backend.Home;

public interface IHomeService
{
    public Task Create(CreateHomeModel home);
    public Task AddUserToHome(Guid homeId, Guid userId);
}

public class HomeService : IHomeService
{
    private readonly Database.AppDbContext _dbContext;

    public HomeService(Database.AppDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async Task Create(CreateHomeModel home)
    {
        var homeModel = new HomeModel
        {
            Name = home.Name,
            Address = home.Address,
            Timezone = home.Timezone
        };
        _dbContext.Homes.Add(homeModel);
        await _dbContext.SaveChangesAsync();
    }

    public async Task AddUserToHome(Guid homeId, Guid userId)
    {
        var home = await _dbContext.Homes.FindAsync(homeId);
        var user = await _dbContext.Users.FindAsync(userId);
        if (home == null || user == null)
        {
            throw new Exception("Home or User not found");
        }
        home.Users.Add(user);
        await _dbContext.SaveChangesAsync();
    }
}
