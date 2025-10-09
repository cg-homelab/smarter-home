// use crate::routes::AppState;
use axum::response::IntoResponse; //{extract::State, response::IntoResponse, Json}
                                  //use lib_models::domain::power::PowerMetrics;
                                  // use lib_models::entity::timeserie::TimeSerieRecord;
use lib_models::error::Error;

pub async fn post_power_metric(// State(state): State<AppState>,
    // Json(input): Json<PowerMetrics>,
) -> impl IntoResponse {
    // let new_metric = lib_models::entity::power::PowerMetrics::from_domain(input.clone());
    // let tx = state.db.questdb_ingress.clone();
    // let res = tx.try_send(TimeSerieRecord::PowerMetrics(new_metric));
    // match res {
    //     Ok(_item) => {
    //         tracing::debug!("Metric saved");
    //         Json(input).into_response()
    //     }
    //     Err(error) => {
    //         tracing::warn!("Metric save failed: {:0}", &error);
    Error::InternalServerError.into_response()
    //     }
    // }
}
