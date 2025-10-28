using Microsoft.EntityFrameworkCore;

using backend.Home;
using backend.User;
using backend.PowerMetric;

namespace backend.Database;

public class AppDbContext : DbContext
{
    public DbSet<HomeModel> Homes { get; set; } = null!;
    public DbSet<UserModel> Users { get; set; } = null!;
    public DbSet<PowerMetricModel> PowerMetrics { get; set; } = null!;

    public AppDbContext(DbContextOptions<AppDbContext> options) : base(options) { }

    // protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder) =>
    //   optionsBuilder

}
