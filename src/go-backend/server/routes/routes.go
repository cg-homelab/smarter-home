package routes

import (
	"smarter-home/server"
	"smarter-home/server/handlers"

	"github.com/gofiber/fiber/v2/middleware/cors"
)

func RegisterFiberRoutes(s *server.FiberServer) {
	handlers := handlers.New()
	s.Handlers = &handlers

	// Apply CORS middleware
	s.App.Use(cors.New(cors.Config{
		AllowOrigins:     "*",
		AllowMethods:     "GET,POST,PUT,DELETE,OPTIONS,PATCH",
		AllowHeaders:     "Accept,Authorization,Content-Type",
		AllowCredentials: false, // credentials require explicit origins
		MaxAge:           300,
	}))

	s.App.Get("/", handlers.HelloWorldHandler)

	s.App.Get("/health", handlers.HealthHandler)
}

// func (s *FiberServer) HelloWorldHandler(c *fiber.Ctx) error {
// 	resp := fiber.Map{
// 		"message": "Hello World",
// 	}
//
// 	return c.JSON(resp)
// }
//
// func (s *FiberServer) healthHandler(c *fiber.Ctx) error {
// 	return c.JSON(s.db.Health())
// }
