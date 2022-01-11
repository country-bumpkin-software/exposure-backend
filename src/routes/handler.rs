use warp::{
    Rejection, 
    Reply, 
    http::StatusCode
};
use chrono::NaiveDateTime;
use tracing::{instrument, event, Level};
use sqlx::postgres::PgPool;
use crate::types;

#[instrument]
pub async fn get_events(db: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "corona-map", Level::INFO, "querying events");
    let rows = sqlx::query!(
        r#"select exposure_id, place_name, address_line1, suburb, postcode, exposure_time_from, exposure_time_to, posted_time from exposure_sites"#
    )
   .fetch_all(&db)
   .await
   .unwrap();

   let record: Vec<types::event::EventDetails> = rows.iter().map(|event_row| types::event::EventDetails {
    exposure_id: event_row.exposure_id.to_string(),
    place_name: event_row.place_name.clone(),
    address_line1: event_row.address_line1.clone(),
    suburb: event_row.suburb.clone(),
    postcode: event_row.postcode.clone(),
    exposure_time_from: Some(event_row.exposure_time_from),
    exposure_time_to: Some(event_row.exposure_time_to),
    posted_time:Some(event_row.posted_time),
   }).collect();

    // let res: types::event::Event = store.events.read().clone();
    event!(target: "corona-map", Level::INFO, Pagination=false);
    Ok(warp::reply::json(&record))
}

pub async fn add_event(db: PgPool, event: types::event::EventDetails) -> Result<impl Reply, Rejection> {
    event!(target: "corona-map", Level::INFO, "querying events");
    sqlx::query!(
        r#"INSERT INTO exposure_sites (exposure_id, place_name, address_line1, suburb, postcode, exposure_time_from, exposure_time_to, posted_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) returning *;"#, &event.exposure_id, event.place_name, event.address_line1,
        event.suburb, event.postcode,event.exposure_time_from, event.exposure_time_to, event.posted_time 
    ).fetch_one(&db).await.unwrap();
    
   
    Ok(warp::reply::with_status("Event added to database", StatusCode::OK))
}

// pub async fn delete_event(id: String, store: store::Store, event: types::event::EventDetails ) -> Result<impl Reply, Rejection> {
//     let delete_id = types::event::EventId(id);
//     store.events.write().event.retain(|x| x.id!=delete_id);
//     Ok(warp::reply::with_status("Event deleted", StatusCode::OK))
// }

