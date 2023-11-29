package models

import (
	"github.com/google/uuid"
)

type User struct {
	Id                       uuid.UUID `gorm:"type:uuid" json:"homeId"`
	FirstName                string    `gorm:"type:text" json:"firstName"`
	LastName                 string    `gorm:"type:text" json:"lastName"`
	PowerTokem               string    `gorm:"type:text" json:"powerToken"`
	PowerProvider            string    `gorm:"type:text" json:"powerProvider"`
	PowerGovSupportThreshold float64   `gorm:"type:double" json:"powerGovSupportThreshold"`
	PowerGovSupportRate      float64   `gorm:"type:double" json:"powerGovSupportRate"`
	PowerTax                 float64   `gorm:"type:double" json:"powerTax"`
	PowerAdditionalCost      float64   `gorm:"type:double" json:"powerAdditionalCost"`
}

type Home struct {
	Id           uuid.UUID `gorm:"type:uuid" json:"id"`
	UserId       uuid.UUID `gorm:"type:uuid" json:"userId"`
	ProviderType string    `gorm:"type:text" json:"providerType"`
	Name         string    `gorm:"type:text" json:"name"`
	WsSupport    bool      `gorm:"type:boolean" json:"wsSupport"`
}
