using System.ComponentModel.DataAnnotations.Schema;
using Microsoft.EntityFrameworkCore;

namespace backend.PowerMetric;

[Keyless]
public class PowerMetricModel
{
    [ForeignKey(nameof(Home))]
    public Guid HomeId { get; set; }
    public DateTime Ts { get; set; }
    public double Price { get; set; }
    public double Power { get; set; }
    public double SolarPower { get; set; }
    public double LastMeterConsumption { get; set; }
    public double LastMeterProduction { get; set; }
    public double ConsumptionSinceMidnight { get; set; }
    public double ProductionSinceMidnight { get; set; }
    public double SolarSinceMidnight { get; set; }
    public double CostSinceMidnight { get; set; }
    public string Currency { get; set; } = string.Empty;
}
