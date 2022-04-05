
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


// ———————————————————————————————————————————————————— REQUESTS ———————————————————————————————————————————————————— //

// SUMMARY: Determines if the light is able to receive a request.
// PARAMS:  Takes the ID of the light to query.
// DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is on
//          and reachable.
// RETURNS: Whether the light is reachable.
fn light_is_reachable(light_id: &str) -> bool
{
	let mut light_info_object: JsonValue = match light_info(light_id)
	{
		Ok(mut light_info_object) => light_info_object,
		Err(_) => return false
	};

	return light_info_object.remove("state").remove("reachable").as_bool().unwrap_or(false);
}


// SUMMARY: Gets the JSON parsed info for a light.
// PARAMS:  Takes the ID of the light to query.
// DETAILS: Makes an HTTP request to the Hub for the light to get the light info. Parses GET request body into a
//          JsonValue.
// RETURNS: Ok(A parsed JsonValue of the light info) or Err(String of the error message).
fn light_info(light_id: &str) -> Result<JsonValue, String>
{
	let url: String = format!("http://{}/api/{}/lights/{}", HUB_URL, API_KEY, light_id);

	let mut response = match reqwest::get(&url)
	{
		Ok(mut response) => response,
		Err(err) => return Err(err.to_string())
	};

	// Check that request was successful
	if(response.status() != reqwest::StatusCode::Ok)
	{
		return Err(format!("HTTP Status Code {} received", response.status()));
	}

	// Set body
	let response_body: String = match response.text()
	{
		Ok(response_body) => response_body,
		Err(err) => return Err(format!("No response body found for get request to {}", response.url()))
	};

	println!("{}", response_body);

	return match parse(&response_body)
	{
		Ok(json_object) => Ok(json_object),
		Err(err) => Err(err.to_string())
	};
}


// SUMMARY: Makes a request to the Hub to configure the light.
// PARAMS:  Takes the value to configure the light color to.
// DETAILS: Makes an HTTP PUT request to the light to set its color.
// RETURNS: Whether the PUT request returns a 200 status
fn set_poweron_color(light_id: &str, poweron_color: &str) -> bool
{
	let url: String = format!("http://{}/api/{}/lights/{}/config", HUB_URL, API_KEY, light_id);
	let body: String = format!("{{\"startup\": {{\"customsettings\": {{{}}}}}}}", poweron_color);

	let put_client = reqwest::Client::new();
	let response = match put_client.put(&url).body(body).send()
	{
		Ok(response) => response,
		Err(_) => return false
	};

	// Check that request was successful
	return response.status() == reqwest::StatusCode::Ok;
}


// ————————————————————————————————————————————————— EVENT PLANNING ————————————————————————————————————————————————— //

// fn sleep_time() -> u32
// {
// 	let now: i64 = Local::now().timestamp();
// 	let start_of_day: i64 = Local::today().and_hms(0, 0, 0).timestamp();
// 	let morning: i64 = Local::today().and_hms(DayTime::HOUR, DayTime::MINUTE, 0).timestamp();
// 	println!("Now: {}", now.to_string());
// 	println!("Start of Day: {}", start_of_day.to_string());
// 	return 1 as u32;
// }


// fn current_time_of_day() -> Time
// {
// 	let current_time: i64 = Local::now().timestamp();
// 	let daytime_start: i64 = Local::today().and_hms(DayTime::HOUR, DayTime::MINUTE, 0).timestamp();
// 	let nighttime_start: i64 = Local::today().and_hms(NightTime::HOUR, NightTime::MINUTE, 0).timestamp();

// 	if(current_time < daytime_start || nighttime_start <= current_time)
// 	{
// 		return Time::DAYTIME;
// 	}
// 	else
// 	{
// 		return Time::NIGHTTIME;
// 	}
// }


fn sleep_for_5_seconds()
{
	let five_seconds = time::Duration::from_millis(5000);
	thread::sleep(five_seconds);
}


fn event_not_in_requests(event: &'static Event::Event, event_requests: &mut Vec<EventRequest::EventRequest>) -> bool
{
	for event_request in event_requests.iter()
	{
		if(event_request.event == event)
		{
			return false;
		}
	}

	return true;
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
}


fn test() -> Vec<EventRequest::EventRequest>
{
	let asdf: Vec<EventRequest::EventRequest> = vec![];

	return asdf;
	// for x in 0..
}



fn main()
{
	let mut upcoming_events: Vec<EventRequest::EventRequest> = vec![];

	loop
	{
		stage_event_requests(&upcoming_events);

		upcoming_events.retain(
			|event_request|
			{
				return !set_poweron_color(event_request.light.value, event_request.event.poweron);
			}
		);

		// while(!light_is_reachable(LIGHT_NUMBER))
		// {
		// 	sleep_for_5_seconds();
		// }

		// let time_of_day: Time = current_time_of_day();
		// let poweron_color: &str = poweron_color_for_time_of_day(time_of_day);

		if(set_poweron_color("3", "HELLO"))
		{
			println!("Light value set");
		}
		else
		{
			println!("Failed to set light value");
		}
		// let poweron_color: &str = if(time_of_day == Time::DAYTIME as u8) {Time::DAYTIME} else {NightTime::VALUE};

		// if(time_of_day() as u8 == Time::DAYTIME as u8)
		// {
		// 	println!("DAYTIME");
		// 	if(set_poweron_color(LIGHT_NUMBER, cE))
		// 	{
		// 		light_is_reachable(LIGHT_NUMBER);
		// 	}
		// 	else
		// 	{
		// 	}
		// }
		// else
		// {
		// 	println!("NIGHTTIME");
		// 	if(set_poweron_color(LIGHT_NUMBER, NightTime::VALUE))
		// 	{
		// 		println!("Light value set");
		// 		light_is_reachable(LIGHT_NUMBER);
		// 	}
		// 	else
		// 	{
		// 		println!("Failed to set light value");
		// 	}
		// }

		break;
	}
}