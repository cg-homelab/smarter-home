package database

import (
	"app/model"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type DBConfig struct {
	ConnectionString string
	LineProto        string
	Type             string
}

func (dbConfig *DBConfig) ConnectDB() (*gorm.DB, error) {
	//
	dsn := dbConfig.ConnectionString

	db, dbErr := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if dbErr != nil {
		return db, dbErr
	}

	db.AutoMigrate(
		&model.User{},
		&model.Home{},
		&model.ConsumptionMetric{},
		&model.ElectricityDeal{},
		&model.ElectricityPrice{})

	// Custom timescale migrations
	createConsumptionHyperTables := "select create_hypertable('consumption_metrics', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);"
	createElectricityPriceHyperTables := "select create_hypertable('electricity_prices', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);"
	db.Exec(createConsumptionHyperTables)
	db.Exec(createElectricityPriceHyperTables)

	return db, nil
}
