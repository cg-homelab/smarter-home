using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace backend.Migrations
{
    /// <inheritdoc />
    public partial class InitialCreate : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            // Check timescale extension is present
            // language=sql
            migrationBuilder.Sql(
                """
                CREATE EXTENSION IF NOT EXISTS timescaledb;
                """
            );

            migrationBuilder.CreateTable(
                name: "Homes",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Name = table.Column<string>(type: "text", nullable: false),
                    Address = table.Column<string>(type: "text", nullable: false),
                    Timezone = table.Column<string>(type: "text", nullable: false),
                    ClientId = table.Column<Guid>(type: "uuid", nullable: false),
                    ClientSecret = table.Column<Guid>(type: "uuid", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Homes", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "PowerMetrics",
                columns: table => new
                {
                    HomeId = table.Column<Guid>(type: "uuid", nullable: false),
                    Ts = table.Column<DateTime>(type: "timestamp with time zone", nullable: false),
                    Price = table.Column<double>(type: "double precision", nullable: false),
                    Power = table.Column<double>(type: "double precision", nullable: false),
                    SolarPower = table.Column<double>(type: "double precision", nullable: false),
                    LastMeterConsumption = table.Column<double>(type: "double precision", nullable: false),
                    LastMeterProduction = table.Column<double>(type: "double precision", nullable: false),
                    ConsumptionSinceMidnight = table.Column<double>(type: "double precision", nullable: false),
                    ProductionSinceMidnight = table.Column<double>(type: "double precision", nullable: false),
                    SolarSinceMidnight = table.Column<double>(type: "double precision", nullable: false),
                    CostSinceMidnight = table.Column<double>(type: "double precision", nullable: false),
                    Currency = table.Column<string>(type: "text", nullable: false)
                },
                constraints: table =>
                {
                });

            // Convert PowerMetrics Table to Hypertable
            // language=sql
            migrationBuilder.Sql(
                """
                SELECT create_hypertable(
                  '"PowerMetrics"',
                  by_range('Ts', INTERVAL '1 month'),
                  if_not_exists=>TRUE
                );
                """
            );

            migrationBuilder.CreateTable(
                name: "Users",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Username = table.Column<string>(type: "text", nullable: false),
                    FirstName = table.Column<string>(type: "text", nullable: false),
                    LastName = table.Column<string>(type: "text", nullable: false),
                    Role = table.Column<int>(type: "integer", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Users", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "HomeModelUserModel",
                columns: table => new
                {
                    HomesId = table.Column<Guid>(type: "uuid", nullable: false),
                    UsersId = table.Column<Guid>(type: "uuid", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_HomeModelUserModel", x => new { x.HomesId, x.UsersId });
                    table.ForeignKey(
                        name: "FK_HomeModelUserModel_Homes_HomesId",
                        column: x => x.HomesId,
                        principalTable: "Homes",
                        principalColumn: "Id",
                        onDelete: ReferentialAction.Cascade);
                    table.ForeignKey(
                        name: "FK_HomeModelUserModel_Users_UsersId",
                        column: x => x.UsersId,
                        principalTable: "Users",
                        principalColumn: "Id",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateIndex(
                name: "IX_HomeModelUserModel_UsersId",
                table: "HomeModelUserModel",
                column: "UsersId");
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "HomeModelUserModel");

            migrationBuilder.DropTable(
                name: "PowerMetrics");

            migrationBuilder.DropTable(
                name: "Homes");

            migrationBuilder.DropTable(
                name: "Users");
        }
    }
}
