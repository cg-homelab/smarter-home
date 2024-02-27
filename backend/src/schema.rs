diesel::table! {
    users(id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        email -> Text,
        password -> Text
    }
}

diesel::table! {
    homes(id) {
        id -> Uuid,
        name -> Text,
        user_id -> Uuid,
        ws_support -> Bool
    }
}
diesel::joinable!(homes -> users (user_id));

diesel::table! {
    electricity_deals(id) {
        id -> Uuid,
        user_id -> Uuid,
        home_id -> Uuid,
        power_provider -> Text,
        power_gov_support_threshold -> Double,
        power_gov_support_rate -> Double,
        power_tax -> Double,
        power_additional_cost -> Double
    }
}
diesel::joinable!(electricity_deals -> users (user_id));
diesel::joinable!(electricity_deals -> homes (home_id));

diesel::table! {
    consumption_metrics(home_id) {
        home_id -> Uuid,
        timestamp -> Timestamptz,
        power -> Double,
        min_power -> Double,
        max_power -> Double,
        average_power -> Double,
        last_meter_consumption -> Double,
        last_meter_production -> Double,
        accumulated_consumption_today -> Double,
        accumulated_production_today -> Double,
        accumulated_consumption_hour -> Double,
        accumulated_production_hour -> Double,
        current_price -> Double,
        accumulated_cost_today -> Double
    }
}
diesel::joinable!(consumption_metrics -> homes (home_id));

diesel::table! {
    electricity_price (home_id) {
        home_id -> Uuid,
        deal_id -> Uuid,
        timestamp -> Timestamptz,
        total_by_provider -> Nullable<Double>,
        spot -> Double,
        tax -> Double,
        calculated -> Double,
        grid -> Double,
        currency -> Text
    }
}
diesel::joinable!(electricity_price -> homes (home_id));
diesel::joinable!(electricity_price -> electricity_deals (deal_id));
