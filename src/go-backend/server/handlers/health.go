package handlers

import "github.com/gofiber/fiber/v2"

func (h *Handlers) HealthHandler(c *fiber.Ctx) error {
	return c.JSON(h.db.Health())
}
