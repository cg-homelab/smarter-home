using System.ComponentModel.DataAnnotations;
using backend.Home;

namespace backend.User;

public enum UserRoles
{
    Admin = 0,
    Owner = 1,
    Viewer = 2,
}

public class UserModel
{
    [Key]
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
    public string FullName => $"{FirstName} {LastName}";
    public UserRoles Role { get; set; } = UserRoles.Viewer;
    public List<HomeModel> Homes { get; } = [];
}

public class UserResponseModel
{
    public Guid Id { get; set; }
    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
    public UserRoles Role { get; set; }
}

public class CreateUserModel
{
    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
}

