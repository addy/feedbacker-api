extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate warp;

use std::sync::{Arc, Mutex};
use warp::{http::StatusCode, Filter};

type Db = Arc<Mutex<Vec<Feedback>>>;

#[derive(Debug, Deserialize, Serialize)]
struct Feedback {
   id: u64,
   title: String,
   text: String,
}

fn main() {
   // Set up in-memory DB
   let db = Arc::new(Mutex::new(Vec::<Feedback>::new()));
   let db = warp::any().map(move || db.clone());
   // Feedback Routes
   let feedback = warp::path("feedback");
   let feedback_index = feedback.and(warp::path::end());
   let feedback_id = feedback
      .and(warp::path::param::<u64>())
      .and(warp::path::end());

   let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

   let list = warp::get2()
      .and(feedback_index)
      .and(db.clone())
      .map(list_feedback);

   let create = warp::post2()
      .and(feedback_index)
      .and(json_body)
      .and(db.clone())
      .and_then(create_feedback);
   let update = warp::put2()
      .and(feedback_id)
      .and(json_body)
      .and(db.clone())
      .and_then(update_feedback);

   let routes = list.or(create).or(update);
   warp::serve(routes).run(([0, 0, 0, 0], 3030));
}

fn list_feedback(db: Db) -> impl warp::Reply {
   warp::reply::json(&*db.lock().unwrap())
}

fn create_feedback(create: Feedback, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
   let mut vec = db.lock().unwrap();

   for feedback in vec.iter() {
      if feedback.id == create.id {
         return Ok(StatusCode::BAD_REQUEST);
      }
   }

   vec.push(create);

   Ok(StatusCode::CREATED)
}

fn update_feedback(id: u64, update: Feedback, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
   let mut vec = db.lock().unwrap();

   for feedback in vec.iter_mut() {
      if feedback.id == id {
         *feedback = update;
         return Ok(warp::reply());
      }
   }

   Err(warp::reject::not_found())
}
