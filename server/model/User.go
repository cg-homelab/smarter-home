package model

import (
	"github.com/google/uuid"
)

type User struct {
	ID               uuid.UUID `gorm:"type:uuid;primeryKey"`
	FirstName        string
	LastName         string
	Username         string
	Email            string
	Password         string
	Homes            []Home            `gorm:"foreignKey:user_id;references:id"`
	ElectricityDeals []ElectricityDeal `gorm:"foreignKey:user_id;references:id"`
}

type Home struct {
	ID           uuid.UUID `gorm:"type:uuid;primeryKey" json:"id"`
	UserId       uuid.UUID `gorm:"type:uuid" json:"userId"`
	ProviderType string    `gorm:"type:text" json:"providerType"`
	Name         string    `gorm:"type:text" json:"name"`
	WsSupport    bool      `gorm:"type:boolean" json:"wsSupport"`
}
