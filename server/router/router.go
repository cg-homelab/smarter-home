package router

import (
	"app/handler"
	//"app/middleware"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/logger"
)

type Router struct {
	AuthHandler handler.AuthHandler
	CMHandler   handler.ConsumptionMetricsHandler
}

// SetupRoutes setup router api
func (r *Router) SetupRoutes(app *fiber.App) {
	// Middleware
	api := app.Group("/api", logger.New())
	//api.Get("/", handler.Hello)

	// Auth
	auth := api.Group("/auth")
	auth.Post("/login", r.AuthHandler.Login)

	// User
	//user := api.Group("/user")
	//user.Get("/:id", handler.GetUser)
	// user.Post("/", handler.CreateUser)
	// user.Patch("/:id", middleware.Protected(), handler.UpdateUser)
	// user.Delete("/:id", middleware.Protected(), handler.DeleteUser)

	// ConsumptionMetrics
	consumptionMetrics := api.Group("/consumption-metrics")
	// consumptionMetrics.Get("/", handler.GetAllProducts)
	// consumptionMetrics.Get("/:id", handler.GetProduct)
	consumptionMetrics.Post("/", r.CMHandler.CreateConsumptionMetrics)
	// consumptionMetrics.Delete("/:id", middleware.Protected(), handler.DeleteProduct)
}
