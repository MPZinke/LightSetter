
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
use std::cmp::Ordering;


use crate::Event::Event;
use crate::Light::Light;
use crate::Config::{EVENTS, LIGHTS};


type Time = i64;


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

	// SUMMARY: Determines if the light is able to receive a request.
	// PARAMS:  Takes the ID of the light to query.
	// DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is
	//          on and reachable.
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

	}


	pub fn should_make_request(&self) -> bool
	{

	}
}
