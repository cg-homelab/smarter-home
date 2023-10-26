package handler

import (
	"app/database"
	"app/models"
	"app/workers"

	"github.com/MartinEllegard/tibber-go"
	"github.com/gofiber/fiber/v2"
)

type CreateConsumptionTrackerInput struct {
	Token string `json:"token"`
}

func CreateConsumptionTracker(c *fiber.Ctx) {
	db := database.QDB
	input := new(CreateConsumptionTrackerInput)
	if err := c.BodyParser(input); err != nil {
		return c.Status(500).JSON(fiber.Map{
			"status":  "error",
			"message": "Could not create Tracker",
			"data":    err,
		})
	}

	worker, err := workers.CreateTibberWorker(input.Token)
	if err == nil {
		worker.Init()
		worker.StartTracking(func(lm tibber.LiveMeasurement, err error) error {
			if (err != nil || lm == tibber.LiveMeasurement{}) {
				return err
			}

			consumption := new(models.SMConsumptionTracker)

			consumption.Timestamp = lm.Timestamp
			consumption.AccumulatedConsumptionHour = float64(lm.AccumulatedConsumptionLastHour)
			consumption.AccumulatedConsumptionToday = float64(lm.AccumulatedConsumption)

			db.Create(consumption)
		})
	}
}

// GetAllProducts query all products
// func GetAllProducts(c *fiber.Ctx) error {
// 	db := database.DB
// 	var products []model.Product
// 	db.Find(&products)
// 	return c.JSON(fiber.Map{"status": "success", "message": "All products", "data": products})
// }

// // GetProduct query product
// func GetProduct(c *fiber.Ctx) error {
// 	id := c.Params("id")
// 	db := database.DB
// 	var product model.Product
// 	db.Find(&product, id)
// 	if product.Title == "" {
// 		return c.Status(404).JSON(fiber.Map{"status": "error", "message": "No product found with ID", "data": nil})

// 	}
// 	return c.JSON(fiber.Map{"status": "success", "message": "Product found", "data": product})
// }

// // CreateProduct new product
// func CreateProduct(c *fiber.Ctx) error {
// 	db := database.DB
// 	product := new(model.Product)
// 	if err := c.BodyParser(product); err != nil {
// 		return c.Status(500).JSON(fiber.Map{"status": "error", "message": "Couldn't create product", "data": err})
// 	}
// 	db.Create(&product)
// 	return c.JSON(fiber.Map{"status": "success", "message": "Created product", "data": product})
// }

// // DeleteProduct delete product
// func DeleteProduct(c *fiber.Ctx) error {
// 	id := c.Params("id")
// 	db := database.DB

// 	var product model.Product
// 	db.First(&product, id)
// 	if product.Title == "" {
// 		return c.Status(404).JSON(fiber.Map{"status": "error", "message": "No product found with ID", "data": nil})

// 	}
// 	db.Delete(&product)
// 	return c.JSON(fiber.Map{"status": "success", "message": "Product successfully deleted", "data": nil})
// }
