use warp::{
    filters::{
        cors::CorsForbidden,
        body::BodyDeserializeError
    }, 
    reject::Reject, 
    Rejection, 
    Reply, 
    http::StatusCode
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    EventNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::EventNotFound => write!(f, "Event not found"),
        }
    }
}
impl Reject for Error {}


pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        println!("{:?}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        println!("{:?}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        println!("{:?}", error);
        Ok(warp::reply::with_status(error.to_string(), StatusCode::UNPROCESSABLE_ENTITY))
    }
     else {
        println!("Route not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}