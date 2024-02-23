package main

import (
	"app/config"
	"app/database"
	"app/handler"
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
		ConnectionString: config.Config("DATABASE_URL"),
	}

	db, dbErr := dbConfig.ConnectDB()
	if dbErr != nil {
		panic(dbErr)
	}

	router := router.Router{
		AuthHandler: handler.CreateAuthHandler(db),
		CMHandler:   handler.CreateCMHandler(db),
	}

	router.SetupRoutes(app)
	log.Fatal(app.Listen(":3000"))
}
