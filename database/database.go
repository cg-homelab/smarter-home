package database

import (
	"app/models"

	qdb "github.com/questdb/go-questdb-client/v2"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// DB gorm connector
var QDB *gorm.DB
var QDBLineSender qdb.LineSender

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

	questDb.AutoMigrate(&models.LiveConsumption{})

	QDB = questDb

	return nil
}
