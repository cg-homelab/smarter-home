-- Your SQL goes here
CREATE TABLE "electricity_deals"(
	"id" UUID NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"home_id" UUID NOT NULL,
	"power_provider" TEXT NOT NULL,
	"power_gov_support_threshold" DOUBLE NOT NULL,
	"power_gov_support_rate" DOUBLE NOT NULL,
	"power_tax" DOUBLE NOT NULL,
	"power_additional_cost" DOUBLE NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id"),
	FOREIGN KEY ("home_id") REFERENCES "homes"("id")
);

CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"deleted_at" TIMESTAMPTZ NOT NULL,
	"first_name" TEXT NOT NULL,
	"last_name" TEXT NOT NULL,
	"username" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL
);

CREATE TABLE "homes"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"user_id" UUID NOT NULL,
	"ws_support" BOOL NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "consumption_metrics"(
	"home_id" UUID NOT NULL,
	"timestamp" TIMESTAMPTZ NOT NULL,
	"power" DOUBLE NOT NULL,
	"min_power" DOUBLE NOT NULL,
	"max_power" DOUBLE NOT NULL,
	"average_power" DOUBLE NOT NULL,
	"last_meter_consumption" DOUBLE NOT NULL,
	"last_meter_production" DOUBLE NOT NULL,
	"accumulated_consumption_today" DOUBLE NOT NULL,
	"accumulated_production_today" DOUBLE NOT NULL,
	"accumulated_consumption_hour" DOUBLE NOT NULL,
	"accumulated_production_hour" DOUBLE NOT NULL,
	"current_price" DOUBLE NOT NULL,
	"accumulated_cost_today" DOUBLE NOT NULL,
	FOREIGN KEY ("home_id") REFERENCES "homes"("home_id")
);
select create_hypertable('consumption_metrics', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);

CREATE TABLE "electricity_price"(
	"home_id" UUID NOT NULL,
	"deal_id" UUID NOT NULL,
	"timestamp" TIMESTAMPTZ NOT NULL,
	"total_by_provider" DOUBLE,
	"spot" DOUBLE NOT NULL,
	"tax" DOUBLE NOT NULL,
	"calculated" DOUBLE NOT NULL,
	"grid" DOUBLE NOT NULL,
	"currency" TEXT NOT NULL,
	FOREIGN KEY ("home_id") REFERENCES "homes"("home_id"),
	FOREIGN KEY ("deal_id") REFERENCES "electricity_deals"("home_id")
);
select create_hypertable('electricity_prices', by_range('timestamp', INTERVAL '1 month'), if_not_exists => TRUE);

