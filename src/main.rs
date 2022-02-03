extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate warp;
extern crate chrono;
extern crate rawsql;

use chrono::prelude::*;
use warp::{Filter};
use postgres::{Connection, TlsMode};
use postgres::rows::Row;
use rawsql::Loader;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Feedback {
   feedback_id: i64,
   feedback_title: String,
   feedback_text: String,
   feedback_score: i32,
   updt_dt_tm: DateTime<Utc>,
   create_dt_tm: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Comment {
   comment_id: i64,
   comment_text: String,
   feedback_id: i64,
   poster_id: i64,
   updt_dt_tm: DateTime<Utc>,
   create_dt_tm: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Poster {
   poster_id: i64,
   poster_name: String,
   pass_hash: String,
   updt_dt_tm: DateTime<Utc>,
   create_dt_tm: DateTime<Utc>,
}

const FEEDBACK_SQL: HashMap<String, String> = Loader::get_queries_from("sql/feedback.sql").unwrap().queries;
const COMMENT_SQL: HashMap<String, String> = Loader::get_queries_from("sql/comment.sql").unwrap().queries;

fn main() {
   // Feedback Routes
   let feedback = warp::path("feedback");
   let feedback_index = feedback.and(warp::path::end());
   let feedback_id = feedback
      .and(warp::path::param::<i64>())
      .and(warp::path::end());

   // Comment Routes
   let comment = warp::path("comment");
   let comment_id = comment
      .and(warp::path::param::<i64>())
      .and(warp::path::end());

   let feedback_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
   let comment_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

   let l_feedback = warp::get2()
      .and(feedback_index)
      .map(list_feedback);
   
   let s_feedback = warp::get2()
      .and(feedback_id)
      .map(show_feedback);

   let c_feedback = warp::post2()
      .and(feedback_index)
      .and(feedback_body)
      .and_then(create_feedback);

   let u_feedback = warp::put2()
      .and(feedback_id)
      .and(feedback_body)
      .and_then(update_feedback);

   let c_comment = warp::post2()
      .and(comment_id)
      .and(comment_body)
      .and_then(create_comment);

   let u_comment = warp::put2()
      .and(comment_id)
      .and(comment_body)
      .and_then(update_comment);

   let feedback_routes = l_feedback.or(s_feedback).or(c_feedback).or(u_feedback);
   let comment_routes = c_comment.or(u_comment);
   let routes = feedback_routes.or(comment_routes);
   warp::serve(routes).run(([0, 0, 0, 0], 3030));
}

fn list_feedback() -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.query(FEEDBACK_SQL.get("list-feedback").unwrap(), &[]) {
      Ok(json) => return Ok(warp::reply::json(&json.iter().map(Feedback::from).collect())),
      _ => Err(warp::reject::not_found()),
   }
}

fn show_feedback(id: i64) -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.query(FEEDBACK_SQL.get("show-feedback").unwrap(), &[&id]) {
      Ok(json) => return Ok(warp::reply::json(&json.iter().map(Feedback::from).collect())),
      _ => Err(warp::reject::not_found()),
   }
}

fn create_feedback(create: Feedback) -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.execute(FEEDBACK_SQL.get("create-feedback").unwrap(), &[&create.feedback_title, &create.feedback_text]) {
      Ok(_) => return Ok(warp::reply()),
      _ => Err(warp::reject::not_found()),
   }
}

fn update_feedback(id: i64, update: Feedback) -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.execute(FEEDBACK_SQL.get("update-feedback").unwrap(), &[&update.feedback_title, &update.feedback_text, &update.feedback_score]) {
      Ok(_) => return Ok(warp::reply()),
      _ => Err(warp::reject::not_found()),
   }
}

fn create_comment(id: i64, create: Comment) -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.execute(COMMENT_SQL.get("create-comment").unwrap(), &[&create.comment_text, &create.feedback_id, &create.poster_id]) {
      Ok(_) => return Ok(warp::reply()),
      _ => Err(warp::reject::not_found()),
   }
}

fn update_comment(id: i64, update: Comment) -> Result<impl warp::Reply, warp::Rejection> {
   let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
   match conn.execute(COMMENT_SQL.get("update-comment").unwrap(), &[&update.comment_text]) {
      Ok(_) => return Ok(warp::reply()),
      _ => Err(warp::reject::not_found()),
   }
}

impl<'a> From<Row<'a>> for Feedback {
   fn from(row: Row) -> Self {
      Feedback {
         feedback_id: row.get(0),
         feedback_title: row.get(1),
         feedback_text: row.get(2),
         feedback_score: row.get(3),
         updt_dt_tm: row.get(4),
         create_dt_tm: row.get(5),
      }
   }
}

impl<'a> From<Row<'a>> for Comment {
   fn from(row: Row) -> Self {
      Comment {
         comment_id: row.get(0),
         comment_text: row.get(1),
         feedback_id: row.get(2),
         poster_id: row.get(3),
         updt_dt_tm: row.get(4),
         create_dt_tm: row.get(5),
      }
   }
}