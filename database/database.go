package database

import (
	"app/models"
	"context"
	"log"
	"time"

	qdb "github.com/questdb/go-questdb-client/v2"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// DB gorm connector
var QDB *gorm.DB
var PDB *gorm.DB
var QDBChannel chan models.LiveConsumption

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

	if dbConfig.Type == "quest" {
		db.AutoMigrate(&models.LiveConsumption{}, &models.ElectricityPrice{})

		sender, senderErr := qdb.NewLineSender(context.Background(), qdb.WithAddress(dbConfig.LineProto))
		if senderErr != nil {
			return senderErr
		}
		ch := make(chan models.LiveConsumption)
		QDBChannel = ch

		// Start qdb line writer
		go func() {
			QdbSenderListener(QDBChannel, sender)
		}()
		QDB = db
	} else {
		db.AutoMigrate(&models.User{}, &models.Home{})
		PDB = db
	}

	return nil
}

func QdbSenderListener(rx <-chan models.LiveConsumption, lineSender *qdb.LineSender) {
	defer lineSender.Close()
	log.Print("Started db worker")

	running := true

	go func() {
		for running {
			time.Sleep(time.Millisecond * 1000)
			lineSender.Flush(context.Background())
		}
	}()

	for message := range rx {
		if (message != models.LiveConsumption{}) {
			err := lineSender.
				Table("live_consumptions").
				Symbol("home_id", message.HomeId).
				TimestampColumn("timestamp", message.Timestamp).
				Float64Column("power", message.Power).
				Float64Column("min_power", message.MinPower).
				Float64Column("max_power", message.MaxPower).
				Float64Column("average_power", message.AveragePower).
				Float64Column("last_meter_consumption", message.LastMeterConsumption).
				Float64Column("last_meter_production", message.LastMeterProduction).
				Float64Column("accumulated_cost_today", message.AccumulatedCostToday).
				Float64Column("accumulated_consumption_hour", message.AccumulatedConsumptionHour).
				Float64Column("accumulated_production_hour", message.AccumulatedProductionHour).
				Float64Column("accumulated_consumption_today", message.AccumulatedConsumptionToday).
				Float64Column("accumulated_production_today", message.AccumulatedProductionToday).
				Float64Column("current_price", message.CurrentPrice).
				At(context.Background(), message.Timestamp)
			if err != nil {
				log.Fatalf("error happend when sendeing message to db: %s", err)
			}

		}

	}
	log.Print("Stopped db worker")
}
