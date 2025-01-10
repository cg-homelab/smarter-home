package server

import (
	"github.com/gofiber/fiber/v2"

	"smarter-home/internal/database"
)

type FiberServer struct {
	*fiber.App

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
