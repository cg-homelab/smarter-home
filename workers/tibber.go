package workers

import (
	"app/database"
	"app/models"
	"log"
	"sync"

	"github.com/MartinEllegard/tibber-go"
)

var TibberWorkers []*TibberWorker = []*TibberWorker{}

const Agent string = "smarter-home/v0.1.0"

type TibberWorker struct {
	Token        string
	tibberClient *tibber.TibberClient
	Running      bool
	View         tibber.Viewer
	WSW          sync.WaitGroup
	apiChannel   chan tibber.LiveMeasurement
	dbChannel    chan models.LiveConsumption
}

func CreateTibberWorker(token string) (*TibberWorker, error) {
	client := tibber.CreateTibberClient(token, Agent)
	worker := &TibberWorker{Token: token, tibberClient: client, Running: false, dbChannel: database.QDBChannel, apiChannel: make(chan tibber.LiveMeasurement)}
	TibberWorkers = append(TibberWorkers, worker)
	return worker, nil
}

func (worker *TibberWorker) Init() error {
	response, err := worker.tibberClient.GetHomes()
	if err != nil {
		log.Fatalf("Error getting homes: %s", err)
		return err
	}

	worker.View = response.Viewer

	return nil
}

func (worker *TibberWorker) StartTracking() error {
	go worker.ProccessMessages(worker.apiChannel)
	counter := 0
	for i := 0; i < len(worker.View.Homes); i++ {
		home := &worker.View.Homes[i]

		if home.Features.RealTimeConsumptionEnabled {
			counter = counter + 1
			go func() {
				log.Printf("Started tibber worker for home: %s", home.ID)
				err := worker.tibberClient.StartSubscription(home.ID, worker.apiChannel)
				log.Fatalf("Failed to start subscription: %s", err)
			}()
		}
	}

	if counter == 0 {
		worker.tibberClient.Close()
	}

	return nil
}
func (worker *TibberWorker) ProccessMessages(channel chan tibber.LiveMeasurement) {
	log.Println("Started tibber message processor")
	for message := range channel {
		log.Println("Got message from tibber")
		if (message != tibber.LiveMeasurement{}) {
			dbObject := models.LiveConsumption{}
			dbObject.HomeId = message.HomeId
			dbObject.CurrentPrice = 0.0
			dbObject.Timestamp = message.Timestamp
			dbObject.AccumulatedCostToday = message.AccumulatedCost
			dbObject.AccumulatedProductionToday = message.AccumulatedProduction
			dbObject.AccumulatedConsumptionToday = message.AccumulatedConsumption
			dbObject.AccumulatedProductionHour = message.AccumulatedProductionLastHour
			dbObject.AccumulatedConsumptionHour = message.AccumulatedConsumptionLastHour
			dbObject.Power = float64(message.Power)
			dbObject.MaxPower = message.MaxPower
			dbObject.MinPower = float64(message.MinPower)
			dbObject.AveragePower = message.AveragePower
			dbObject.LastMeterProduction = message.LastMeterProduction
			dbObject.LastMeterConsumption = message.LastMeterConsumption

			worker.dbChannel <- dbObject
		}
	}

	log.Println("Stopped tibber message processor")
}

func CloseConnection() {
	for _, worker := range TibberWorkers {
		worker.tibberClient.Close()
	}
}
