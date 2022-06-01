
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use chrono::{DateTime, Date, Local};
use json::{JsonValue, parse};
use reqwest;
use std::cmp::Ordering;


use crate::Event::Event;
use crate::Light::Light;
use crate::Config::{EVENTS, LIGHTS};


type Time = i64;


static HUB_URL: &str = env!("HUB_URL");
static API_KEY: &str = env!("API_KEY");


pub struct EventRequest
{
	pub event: &'static Event,
	pub light: &'static Light,
	pub timestamp: i64,
}


impl EventRequest
{
	pub fn new(event: &'static Event) -> EventRequest
	{
		for light in LIGHTS.iter()
		{
			if(light.id == event.light)
			{
				return EventRequest{event: event, light: light, timestamp: event.next()};
			}
		}

		panic!("Could not find light for ID");
	}


	// ———————————————————————————————————————————————————— TIME ———————————————————————————————————————————————————— //

	pub fn compare(&self, b: &EventRequest) -> Ordering
	{
		let self_time_remaining: Time = self.time_remaining();
		let b_time_remaining: Time = b.time_remaining();

		if(self_time_remaining < b_time_remaining)
		{
			return Ordering::Less;
		}
		if(self_time_remaining == b_time_remaining)
		{
			return Ordering::Equal;
		}
		else
		{
			return Ordering::Greater;
		}
	}


	pub fn time_remaining(&self) -> Time
	{
		let current_time: Time = Local::now().timestamp();
		return self.timestamp - current_time;
	}


	// —————————————————————————————————————————————————— REQUESTS —————————————————————————————————————————————————— //

	// SUMMARY: Makes a request to the Hub to configure the light.
	// PARAMS:  Takes the value to configure the light color to.
	// DETAILS: Makes an HTTP PUT request to the light to set its color.
	// RETURNS: Whether the PUT request returns a 200 status
	pub fn run(&self) -> bool
	{
		let url: String = format!("http://{}/api/{}/lights/{}/config", HUB_URL, API_KEY, self.light.value);
		let body: String = format!("{{\"startup\": {{\"customsettings\": {{{}}}}}}}", self.event.poweron);

		let put_client = reqwest::Client::new();
		let response = match put_client.put(&url).body(body).send()
		{
			Ok(response) => response,
			Err(_) => return false
		};

		// Check that request was successful
		return response.status() == reqwest::StatusCode::Ok;
	}


	pub fn make_request(&self) -> bool
	{
		return false;
	}


	pub fn should_make_request(&self) -> bool
	{
		return false;
	}
}
