
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


// fn to_time(timestamp: i64) -> String
// {
// 	use chrono::{DateTime, NaiveDateTime, Utc};
// 	let nanoseconds = 230 * 1000000;
//	let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, nanoseconds), Utc);
// 	return format!("{}", datetime);
// }


// fn to_hms(delta: Seconds) -> String
// {
// 	let hours = delta / 3600;
// 	let minutes = (delta - (hours * 3600)) / 60;
// 	let seconds = delta % 60;

// 	return format!("Hours: {}, Minutes: {}, Seconds: {}", hours, minutes, seconds);
// }


// ———————————————————————————————————————————————— EVENT  COMPILING ———————————————————————————————————————————————— //

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
		run_event_requests(&bridge_ip, &mut event_requests).await;

		recompile_event_requests(&connection_pool, &mut event_requests).await;
		
		// Determine sleeptime & sleep
		let sleep_time = calculate_sleep_time(&event_requests);
		sleep_for_(sleep_time);
	}
}