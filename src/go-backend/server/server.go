package server

import (
	"smarter-home/server/handlers"

	"github.com/gofiber/fiber/v2"

	database "smarter-home/server/db"
)

type FiberServer struct {
	*fiber.App

	Handlers *handlers.Handlers

	db database.Service
}

func New() *FiberServer {
	server := &FiberServer{
		App: fiber.New(fiber.Config{
			ServerHeader: "smarter-home",
			AppName:      "smarter-home",
		}),

		db: database.New(),
	}

	return server
}
