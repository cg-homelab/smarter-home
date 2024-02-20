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

	dbConfig := database.DBConfig{
		ConnectionString: config.Config("PDBWIRE"),
	}

	dbErr := dbConfig.ConnectDB()
	if dbErr != nil {
		panic(dbErr)
	}

	router.SetupRoutes(app)
	log.Fatal(app.Listen(":3000"))
}
