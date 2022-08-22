
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


use crate::Query::{new_connection_pool, Event::SELECT_Events};


use crate::Types::EventRequest::EventRequest;
use crate::Types::Event::Event;


type Seconds = u64;
type Timestamp = i64;


// ———————————————————————————————————————————————— EVENT  COMPILING ———————————————————————————————————————————————— //

fn add_missing_events(event_requests: &mut Vec<EventRequest>, events: &Vec<Event>) -> ()
{
	for x in 0..events.len()
	{
		if(event_requests.iter().all(|request| request.event != events[x]))
		{
			event_requests.push(EventRequest::new(events[x].clone()));
		}
	}
}


/*
SUMMARY: Edits the event_requests after a change has been made.
PARAMS:  Takes the connection pool to query the DB from, the event requests vector to edit.
DETAILS: Gets the Events from the DB. Removes all event requests that no longer have any implication (activated &
         obsolete). Then removes all event requests that are overridden by more recently fail event requests for a
         light. These more recently failed event requests for a light will then be retried over and over until success
         or they are overridden.
*/
async fn recompile_event_requests(connection_pool: &PgPool, event_requests: &mut Vec<EventRequest>) -> ()
{
	let events: Vec<Event> = match(SELECT_Events(connection_pool).await)
	{
		Ok(events) => events,
		Err(error) => {println!("{}", error); vec![]}
	};

	// Remove-activated requests
	event_requests.retain(|event_request| event_request.is_activated == false);
	// Remove event requests that have obsolete events.
	event_requests.retain(|event_request| event_request.has_up_to_date_event(&events));

	// Remove all that can be replaced by a more recently attempted event for the light
	let requests_clone: Vec<EventRequest> = event_requests.clone();
	event_requests.retain(|event_request| event_request.is_not_superseded_by_more_recent_light_event(&requests_clone));
	add_missing_events(event_requests, &events);  // Add fresh event requests that were removed for being supersceded
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


// —————————————————————————————————————————————————————— TIME —————————————————————————————————————————————————————— //

fn calculate_sleep_time(event_requests: &Vec<EventRequest>) -> Seconds
{
	if(event_requests.len() == 0)
	{
		return 30;
	}

	let mut soonest_timestamp: Timestamp = event_requests[0].timestamp;
	for event_request in event_requests.iter()
	{
		if(event_request.timestamp < soonest_timestamp)
		{
			soonest_timestamp = event_request.timestamp;
		}
	}

	let current_timestamp: Timestamp = Local::now().timestamp();
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
		// Ensure that everything is accurate after sleep time
		recompile_event_requests(&connection_pool, &mut event_requests).await;
		run_event_requests(&bridge_ip, &mut event_requests).await;
		// Things have changed: recompile for accurate information
		recompile_event_requests(&connection_pool, &mut event_requests).await;

		// Determine sleeptime & sleep
		let sleep_time = calculate_sleep_time(&event_requests);
		sleep_for_(sleep_time);
	}
}