namespace SmarterHome.Api.Repository;
public interface IUserService
{
    Task<User> GetUserByIdAsync(Guid userId);
    Task<IEnumerable<User>> GetAllUsersAsync();
    Task AddUserAsync(User user);
    Task UpdateUserAsync(User user);
    Task DeleteUserAsync(Guid userId);
}

