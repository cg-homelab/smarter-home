package handler

import "gorm.io/gorm"

type ApiHandler interface {
	ConsumptionMetricsHandler
	AuthHandler
}

func CreateAuthHandler(db *gorm.DB) AuthHandler {
	return AuthHandler{db}
}

func CreateCMHandler(db *gorm.DB) ConsumptionMetricsHandler {
	return ConsumptionMetricsHandler{db}
}

func CreateUserHandler(db *gorm.DB) UserHandler {
	return UserHandler{db}
}
