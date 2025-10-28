namespace backend.User;

//create minimal api endpoints group
public static class UserEndpoints
{
    public static void MapUserEndpoints(this WebApplication app)
    {
        var group = app.MapGroup("/api/users").WithTags("Users");

        group.MapPost("/", async (CreateUserModel createUserModel, Database.AppDbContext dbContext) =>
        {
            var user = new UserModel
            {
                Username = createUserModel.Username,
                FirstName = createUserModel.FirstName,
                LastName = createUserModel.LastName,
            };
            dbContext.Users.Add(user);
            await dbContext.SaveChangesAsync();
            return Results.Created($"/api/users/{user.Id}", new UserResponseModel
            {
                Id = user.Id,
                Username = user.Username,
                FirstName = user.FirstName,
                LastName = user.LastName,
                Role = user.Role,
            });
        });
        // group.MapGet("/{id:guid}", async (Guid id, UserService userService) =>
        // {
        //     var user = await userService.Get(id);
        //     if (user == null)
        //     {
        //         return Results.NotFound();
        //     }
        //     return Results.Ok(new UserResponseModel
        //     {
        //         Id = user.Id,
        //         Username = user.Username,
        //         FirstName = user.FirstName,
        //         LastName = user.LastName,
        //         Role = user.Role,
        //     });
        // });
    }
}
