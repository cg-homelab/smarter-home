package main

import (
	"app/config"
	"app/database"
	"app/router"
	"log"

	"github.com/gofiber/fiber/v2"
	// "github.com/gofiber/fiber/v2/middleware/cors"
)

func main() {
	app := fiber.New(fiber.Config{
		Prefork:       true,
		CaseSensitive: true,
		StrictRouting: true,
		ServerHeader:  "Fiber",
		AppName:       "Smarter Home",
	})
	// app.Use(cors.New())

	qdbConfig := database.DBConfig{
		ConnectionString: config.Config("QDBWIRE"),
		LineProto:        config.Config("QDBINFLUX"),
		Type:             "quest",
	}

	pdbConfig := database.DBConfig{
		ConnectionString: config.Config("PDBWIRE"),
	}

	dberr1 := qdbConfig.ConnectDB()
	dberr2 := pdbConfig.ConnectDB()
	if dberr1 != nil {
		panic(dberr1)
	}
	if dberr2 != nil {
		panic(dberr2)
	}

	router.SetupRoutes(app)
	log.Fatal(app.Listen(":3000"))
}
