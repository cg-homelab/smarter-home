package model

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type User struct {
	ID               uuid.UUID `gorm:"type:uuid;primeryKey"`
	CreatedAt        time.Time
	UpdatedAt        time.Time
	DeletedAt        gorm.DeletedAt `gorm:"index"`
	FirstName        string
	LastName         string
	Username         string            `gorm:"uniqueIndex;not null;size:50;" validate:"required,min=3,max=50" json:"username"`
	Email            string            `gorm:"uniqueIndex;not null;size:255;" validate:"required,email" json:"email"`
	Password         string            `gorm:"not null;" validate:"required,min=6,max=50" json:"password"`
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
