package models

import (
	"time"
)

// Product struct
type PriceInfo struct {
	HomeId        string    `gorm:"type:sample" json:"homeId"`
	Timestamp     time.Time `gorm:"type:timestamp" json:"timestamp"`
	TotalProvider float64   `gorm:"type:double" json:"totalProvider"`
	Energy        float64   `gorm:"type:double" json:"energy"`
	Tax           float64   `gorm:"type:double" json:"tax"`
	Currency      string    `gorm:"type:symbol" json:"currency"`
}
