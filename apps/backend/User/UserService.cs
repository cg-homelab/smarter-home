using Microsoft.EntityFrameworkCore;

namespace backend.User;

public interface IUserService
{
    Task<UserResponseModel?> Get(string username);
    Task Create(CreateUserModel user);
}

public class UserService : IUserService
{
    private readonly Database.AppDbContext _dbContext;

    public UserService(Database.AppDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async Task<UserResponseModel?> Get(string username)
    {
        var user = await _dbContext.Users
            .Where(u => u.Username == username)
            .Select(u => new UserResponseModel
            {
                Id = u.Id,
                Username = u.Username,
                FirstName = u.FirstName,
                LastName = u.LastName,
                Role = u.Role
            })
            .FirstOrDefaultAsync();

        return user;
    }

    public async Task Create(CreateUserModel user)
    {
        var userModel = new UserModel
        {
            Username = user.Username,
            FirstName = user.FirstName,
            LastName = user.LastName
        };

        _dbContext.Add<UserModel>(userModel);

        await _dbContext.SaveChangesAsync();
    }
}
