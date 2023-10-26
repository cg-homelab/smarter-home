package models

import (
	"time"
)

// Product struct
type SMConsumptionTracker struct {
	HomeId                      string    `gorm:"type:sample" json:"homeId"`
	Timestamp                   time.Time `gorm:"type:timestamp" json:"timestamp"`
	Power                       float64   `gorm:"type:double" json:"power"`
	MinPower                    float64   `gorm:"type:double" json:"minPower"`
	MaxPower                    float64   `gorm:"type:double" json:"maxPower"`
	AveragePower                float64   `gorm:"type:double" json:"averagePower"`
	LastMeterConsumption        float64   `gorm:"type:double" json:"lastMeterConsuption"`
	LastMeterProduction         float64   `gorm:"type:double" json:"lastMeterProduction"`
	AccumulatedConsumptionToday float64   `gorm:"type:double" json:"accumulatedConsumptionToday"`
	AccumulatedProductionToday  float64   `gorm:"type:double" json:"accumulatedProductionToday"`
	AccumulatedConsumptionHour  float64   `gorm:"type:double" json:"accumulatedConsumptionHour"`
	AccumulatedProductionHour   float64   `gorm:"type:double" json:"accumulatedProductionHour"`
	AccumulatedCostToday        float64   `gorm:"type:double" json:"accumulatedCostToday"`
}
