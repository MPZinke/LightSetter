
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
use json::{JsonValue, parse};
use reqwest;
use std::{thread, time};


mod Config;

mod Event;
mod Light;
mod EventRequest;


use crate::Config::{EVENTS};
// use crate::EventRequest::EventRequest;


static HUB_URL: &str = env!("HUB_URL");
static API_KEY: &str = env!("API_KEY");


type Seconds = u64;
type Time = i64;


// ———————————————————————————————————————————————————— REQUESTS ———————————————————————————————————————————————————— //


// ————————————————————————————————————————————————— EVENT PLANNING ————————————————————————————————————————————————— //

fn sleep_for_(seconds: Seconds) -> ()
{
	let sleep_time: std::time::Duration = time::Duration::from_millis(seconds * 1000);
	thread::sleep(sleep_time);
}


fn sleep_time(event_requests: &Vec<EventRequest::EventRequest>) -> Seconds
{
	assert!(event_requests.len() > 1);


	let current_time: Time = Local::now().timestamp();
	let soonest_time: Time = event_requests[0].timestamp();

	// If event has occurred or will occur in the next 5 seconds
	if(soonest_time < current_time + 5)
	{
		return 5;
	}
	else
	{
		return (soonest_time - current_time) as Seconds;
	}
}


fn stage_event_requests(event_requests: &mut Vec<EventRequest::EventRequest>) -> ()
{
	for event in Config::EVENTS.iter()
	{
		if(event.not_in_requests(event_requests))
		{
			event_requests.push(EventRequest::EventRequest::new(event));
		}
	}
	upcoming_events.sort_by(|a, b| a.compare(b));
}


fn main()
{
	let mut upcoming_events: Vec<EventRequest::EventRequest> = vec![];

	loop
	{
		// Stage events
		stage_event_requests(&mut upcoming_events);

		// Determine sleep & sleep until



		// Call events & retain the ones that are unsuccessfull
		upcoming_events.retain(
			|event_request|
			{
				if(event_request.timestamp < current_time)
				!set_poweron_color(event_request.light.value, event_request.event.poweron)
			}
		);


		break;
	}
}