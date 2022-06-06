
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


use chrono::Local;
use sqlx::PgPool;
use std::{thread, time};


mod LookupError;
mod Requests;
mod Query;
mod Types;


use crate::Query::Event::SELECT_Events;


use crate::Types::EventRequest::EventRequest;
use crate::Types::Event::Event;


type Seconds = u64;
type Timestamp = i64;
// type Time = i64;


async fn new_connection_pool() -> PgPool
{
	let host: &str = "localhost";
	// let user: &str = "root";
	let user: &str = "mpzinke";
	let DB_name: &str = "LightSetter";

	let connection_str: String = format!("postgres://{}@{}:5432/{}", user, host, DB_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str).await
	  .expect("Failed to create Postgres DB connection pool");

	return connection_pool;
}



// ———————————————————————————————————————————————————— REQUESTS ———————————————————————————————————————————————————— //

// async fn compile_event_requests(connection_pool: &PgPool) -> Result<Vec<EventRequest>, LookupError::LookupError>
// {
// 	let events: Vec<Event> = match(SELECT_Events(connection_pool).await)
// 	{
// 		Ok(events) => events,
// 		Err(error) => return Err(error)
// 	};

// 	let mut event_requests: Vec<EventRequest> = vec![];
// 	for event in events.iter()
// 	{
// 		event_requests.push(EventRequest::new(event.clone()));
// 	}
// 	return Ok(event_requests);
// }


// fn remaining_event_requests(previous_event_requests: Vec<EventRequest>) -> Vec<EventRequest>
// {
// 	// Transfer remaining events
// 	let mut upcoming_event_requests: Vec<EventRequest> = vec![];
// 	for previous_event_request in previous_event_requests.iter()
// 	{
// 		if(previous_event_request.is_activated == false)
// 		{
// 			upcoming_event_requests.push((*previous_event_request).clone());
// 		}
// 	}

// 	return upcoming_event_requests;
// }


fn add_missing_events(event_requests: &mut Vec<EventRequest>, events: &Vec<Event>) -> ()
{
	for x in 0..events.len()
	{
		let mut event_is_not_event_requests: bool = true;
		for event_request in event_requests.iter()
		{
			if(event_request.event.id == events[x].id)
			{
				event_is_not_event_requests = false;
			}
		}

		if(event_is_not_event_requests)
		{
			event_requests.push(EventRequest::new(events[x].clone()));
		}
	}
}


async fn run_event_requests(bridge_ip: &String, event_requests: &mut Vec<EventRequest>) -> ()
{
	for x in 0..event_requests.len()
	{
		if(event_requests[x].should_run())
		{
			event_requests[x].run(bridge_ip).await;
		}
	}
}


async fn recompile_event_requests(connection_pool: &PgPool, event_requests: &mut Vec<EventRequest>) -> ()
{
	event_requests.retain(|event_request| event_request.is_activated == false);  // Keep non-activated requests

	let updated_events: Vec<Event> = match(SELECT_Events(connection_pool).await)
	{
		Ok(events) => events,
		Err(error) => {println!("{}", error); vec![]}
	};
	event_requests.retain(|event_request| event_request.has_up_to_date_event(&updated_events));

	add_missing_events(event_requests, &updated_events);
}


// ————————————————————————————————————————————————— EVENT PLANNING ————————————————————————————————————————————————— //

fn calculate_sleep_time(event_requests: &Vec<EventRequest>) -> Seconds
{
	if(event_requests.len() == 0)
	{
		return 30;
	}

	let current_timestamp: Timestamp = Local::now().timestamp();
	let soonest_timestamp: Timestamp = event_requests[0].timestamp;

	// If event has occurred or will occur in the next 5 seconds
	if(soonest_timestamp < current_timestamp + 5)
	{
		return 5;
	}
	else
	{
		return (soonest_timestamp - current_timestamp) as Seconds;
	}
}


fn sleep_for_(seconds: Seconds) -> ()
{
	let sleep_time: std::time::Duration = time::Duration::from_millis(seconds * 1000);
	thread::sleep(sleep_time);
}


#[tokio::main]
async fn main()
{
	let bridge_ip: String = Requests::get_hue_bridge_IP().await;
	let connection_pool: PgPool = new_connection_pool().await;
	let mut event_requests: Vec<EventRequest> = vec![];

	loop
	{
		run_event_requests(&bridge_ip, &mut event_requests).await;

		// add_missing_events(&connection_pool, &mut event_requests).await;
		// update_changed_events(&mut event_requests);
		recompile_event_requests(&connection_pool, &mut event_requests).await;
		
		// Determine sleeptime & sleep
		let sleep_time = calculate_sleep_time(&event_requests);
		sleep_for_(sleep_time);
	}

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