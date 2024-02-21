package database

import (
	"app/models"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// DB gorm connector
var DB *gorm.DB

type DBConfig struct {
	ConnectionString string
	LineProto        string
	Type             string
}

func (dbConfig *DBConfig) ConnectDB() error {
	//
	dsn := dbConfig.ConnectionString

	db, dbErr := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if dbErr != nil {
		return dbErr
	}

	db.AutoMigrate(&models.User{}, &models.Home{}, &models.ConsumptionMetric{}, &models.ElectricityDeal{}, &models.ElectricityPrice{})

	// Custom timescale migrations
	createConsumptionHyperTables := "select create_hypertable('consumption_metrics', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);"
	createElectricityPriceHyperTables := "select create_hypertable('electricity_prices', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);"
	db.Exec(createConsumptionHyperTables)
	db.Exec(createElectricityPriceHyperTables)

	DB = db

	return nil
}
