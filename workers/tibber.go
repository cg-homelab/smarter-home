package workers

import (
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
}

func CreateTibberWorker(token string) (*TibberWorker, error) {
	client := tibber.CreateTibberClient(token, Agent)
	worker := &TibberWorker{Token: token, tibberClient: client, Running: false}
	TibberWorkers = append(TibberWorkers, worker)
	return worker, nil
}

func (worker *TibberWorker) Init() error {
	response, err := worker.tibberClient.GetHomes()
	if err != nil {
		return err
	}

	worker.View = response.Viewer

	return nil
}

func (worker *TibberWorker) StartTracking(fn tibber.SubscriptionHandler) error {
	for i := 0; i < len(worker.View.Homes); i++ {
		home := &worker.View.Homes[i]

		if home.Features.RealTimeConsumptionEnabled {
			worker.WSW.Add(1)

			go func() {
				defer worker.WSW.Done()
				err := worker.tibberClient.StartSubscription(home.ID, fn)
				log.Fatalf("Failed to start subscription: %s", err)
			}()
		}
	}

	go func() {
		defer worker.tibberClient.Close()
		worker.WSW.Wait()
	}()

	return nil
}
