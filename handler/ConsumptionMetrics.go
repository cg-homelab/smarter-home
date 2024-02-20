package handler

import (
	"time"

	"app/database"
	"app/models"
	"github.com/gofiber/fiber/v2"
)

type ConsumptionMetricInput struct {
	Data []models.ConsumptionMetric `json:"data"`
}

// Save ConsumptionMetrics
func CreateConsumptionMetrics(c *fiber.Ctx) error {
	db := database.DB

	input := new(ConsumptionMetricInput)

	if err := c.BodyParser(input); err != nil {
		return c.Status(500).JSON(fiber.Map{
			"status":  "error",
			"message": "Could not parse input data",
			"data":    err,
		})
	}

	result := db.Create(&input.Data)
	if result.Error != nil {
		return c.JSON(fiber.Map{
			"status":  "error",
			"message": "Could not insert data in database",
			"data":    result.Error,
		})
	}

	return c.JSON(fiber.Map{"status": "success", "message": "Data Inserted correctly", "data": ""})
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
