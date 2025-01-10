package handlers

import (
	database "smarter-home/server/db"

	"github.com/gofiber/fiber/v2"
)

type Handlers struct {
	db database.Service
}

func New() Handlers {
	handlers := Handlers{
		db: database.New(),
	}
	return handlers
}

func (h *Handlers) HelloWorldHandler(c *fiber.Ctx) error {
	resp := fiber.Map{
		"message": "Hello World",
	}

	return c.JSON(resp)
}
