package database

import (
	"app/models"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// DB gorm connector
var QDB *gorm.DB

type QDBConfig struct {
	ConnectionString string
}

func (dbConfig *QDBConfig) ConnectQDB() error {
	//
	dsn := dbConfig.ConnectionString

	questDb, qdberr := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if qdberr != nil {
		return qdberr
	}

	questDb.AutoMigrate(&models.SMConsumptionTracker{})

	QDB = questDb

	return nil
}
