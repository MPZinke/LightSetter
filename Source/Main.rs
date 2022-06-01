
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.03.20                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


#![allow(non_snake_case)]
#![allow(unused_parens)]


use chrono::{DateTime, Date, Local};
use reqwest;
use std::{thread, time};


mod LookupError;
mod Requests;
mod Query;
mod Types;
// mod Config;

// mod Event;
// mod Light;
// mod EventRequest;


// use crate::Config::{EVENTS};
// // use crate::EventRequest::EventRequest;


// type Seconds = u64;
// type Time = i64;


// // ———————————————————————————————————————————————————— REQUESTS ———————————————————————————————————————————————————— //


// // ————————————————————————————————————————————————— EVENT PLANNING ————————————————————————————————————————————————— //

// fn sleep_for_(seconds: Seconds) -> ()
// {
// 	let sleep_time: std::time::Duration = time::Duration::from_millis(seconds * 1000);
// 	thread::sleep(sleep_time);
// }


// fn sleep_time(event_requests: &Vec<EventRequest::EventRequest>) -> Seconds
// {
// 	assert!(event_requests.len() > 1);


// 	let current_time: Time = Local::now().timestamp();
// 	let soonest_time: Time = event_requests[0].timestamp;

// 	// If event has occurred or will occur in the next 5 seconds
// 	if(soonest_time < current_time + 5)
// 	{
// 		return 5;
// 	}
// 	else
// 	{
// 		return (soonest_time - current_time) as Seconds;
// 	}
// }


// fn stage_event_requests(event_requests: &mut Vec<EventRequest::EventRequest>) -> ()
// {
// 	for event in Config::EVENTS.iter()
// 	{
// 		if(event.not_in_requests(event_requests))
// 		{
// 			event_requests.push(EventRequest::EventRequest::new(event));
// 		}
// 	}
// 	upcoming_events.sort_by(|a, b| a.compare(b));
// }


#[tokio::main]
async fn main()
{
	let bridge_ip: String = Requests::get_hue_bridge_IP().await;
	println!("HueBridge: {}", bridge_ip);

	let host: &str = "localhost";
	// let user: &str = "root";
	let user: &str = "mpzinke";
	let DB_name: &str = "LightSetter";

	let connection_str: String = format!("postgres://{}@{}:5432/{}", user, host, DB_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str).await
	  .expect("Failed to create Postgres DB connection pool");


	// let mut upcoming_events: Vec<EventRequest::EventRequest> = vec![];

	// loop
	// {
	// 	// Stage events
	// 	stage_event_requests(&mut upcoming_events);

	// 	// Determine sleep & sleep until



	// 	// Call events & retain the ones that are unsuccessfull
	// 	upcoming_events.retain(
	// 		|event_request|
	// 		{
	// 			!event_request.run()
	// 		}
	// 	);


	// 	break;
	// }
}