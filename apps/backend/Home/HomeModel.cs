using System.ComponentModel.DataAnnotations;
using backend.User;

namespace backend.Home;

public class HomeModel
{
    [Key]
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Name { get; set; } = string.Empty;
    public string Address { get; set; } = string.Empty;
    public string Timezone { get; set; } = string.Empty;
    public Guid ClientId { get; set; } = Guid.NewGuid();
    public Guid ClientSecret { get; set; } = Guid.NewGuid();
    public List<UserModel> Users { get; } = [];
}

public class CreateHomeModel
{
    public string Name { get; set; } = string.Empty;
    public string Address { get; set; } = string.Empty;
    public string Timezone { get; set; } = string.Empty;
}
