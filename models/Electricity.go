package models

import (
	"time"

	"github.com/google/uuid"
)

type ElectricityDeal struct {
	ID                       uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4();primeryKey" json:"id"`
	UserID                   uuid.UUID `json:"userId"`
	PowerProvider            string    `json:"powerProvider"`
	PowerGovSupportThreshold float64   `json:"powerGovSupportThreshold"`
	PowerGovSupportRate      float64   `json:"powerGovSupportRate"`
	PowerTax                 float64   `json:"powerTax"`
	PowerAdditionalCost      float64   `json:"powerAdditionalCost"`
}

type ConsumptionMetric struct {
	HomeId                      uuid.UUID `gorm:"type:uuid" json:"homeId"`
	Timestamp                   time.Time `gorm:"type:timestamptz" json:"timestamp"`
	Power                       float64   `json:"power"`
	MinPower                    float64   `json:"minPower"`
	MaxPower                    float64   `json:"maxPower"`
	AveragePower                float64   `json:"averagePower"`
	LastMeterConsumption        float64   `json:"lastMeterConsuption"`
	LastMeterProduction         float64   `json:"lastMeterProduction"`
	AccumulatedConsumptionToday float64   `json:"accumulatedConsumptionToday"`
	AccumulatedProductionToday  float64   `json:"accumulatedProductionToday"`
	AccumulatedConsumptionHour  float64   `json:"accumulatedConsumptionHour"`
	AccumulatedProductionHour   float64   `json:"accumulatedProductionHour"`
	CurrentPrice                float64   `json:"CurrentPrice"`
	AccumulatedCostToday        float64   `json:"accumulatedCostToday"`
}

type ElectricityPrice struct {
	HomeId          uuid.UUID `gorm:"type:uuid" json:"homeId"`
	Timestamp       time.Time `gorm:"type:timestamptz" json:"timestamp"`
	TotalByProvider float64   `json:"totalProvider"`
	Spot            float64   `json:"energy"`
	Tax             float64   `json:"tax"`
	Calculated      float64   `json:"calculated"`
	Grid            float64   `json:"grid"`
	Currency        string    `json:"currency"`
}
