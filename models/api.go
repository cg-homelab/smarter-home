package models

import (
	"time"
)

// Product struct
type ElectricityPrice struct {
	HomeId          string    `gorm:"type:symbol" json:"homeId"`
	Timestamp       time.Time `gorm:"type:timestamp" json:"timestamp"`
	TotalByProvider float64   `gorm:"type:double" json:"totalProvider"`
	Spot            float64   `gorm:"type:double" json:"energy"`
	Tax             float64   `gorm:"type:double" json:"tax"`
	Calculated      float64   `gorm:"type:double" json:"calculated"`
	Grid            float64   `gorm:"type:double" json:"grid"`
	Currency        string    `gorm:"type:symbol" json:"currency"`
}
