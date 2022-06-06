
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


use chrono::Local;
use reqwest;
use std::cmp::Ordering;


use crate::Types::Event::Event;


type Timestamp = i64;


static API_KEY: &str = env!("LIGHTSETTER_APIKEY");


#[derive(Clone)]
pub struct EventRequest
{
	pub event: Event,
	pub is_activated: bool,  // whether the request has been activated
	pub timestamp: i64,  // timestamp of this request's activation
}


impl EventRequest
{
	pub fn new(event: Event) -> EventRequest
	{
		return EventRequest{timestamp: event.next_timestamp(), is_activated: false, event: event};
	}


	pub fn has_up_to_date_event(&self, events: &Vec<Event>) -> bool
	{
		for event in events.iter()
		{
			if(self.event.id == event.id)
			{
				return self.event == (*event);
			}
		}
		return false;  // not found in the events, so remove from event requests
	}


	// ———————————————————————————————————————————————————— TIME ———————————————————————————————————————————————————— //

	pub fn compare(&self, b: &EventRequest) -> Ordering
	{
		let self_time_remaining: Timestamp = self.time_remaining();
		let b_time_remaining: Timestamp = b.time_remaining();

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


	/*
	SUMMARY: Determines the remaining time until the event (or elapsed time if already passed).
	DETAILS: Subtracts the EventRequest's timestamp from the current timestamp.
	*/
	pub fn time_remaining(&self) -> Timestamp
	{
		let current_timestamp: Timestamp = Local::now().timestamp();
		return self.timestamp - current_timestamp;
	}


	// —————————————————————————————————————————————————— REQUESTS —————————————————————————————————————————————————— //

	/*
	SUMMARY: Makes a request to the Hub to configure the light.
	PARAMS:  Takes the value to configure the light color to.
	DETAILS: Makes an HTTP PUT request to the light to set its color.
	RETURNS: Whether the PUT request returns a 200 status
	*/
	pub async fn run(&mut self, hub_url: &String) -> ()
	{
		let url: String = format!("http://{}/api/{}/lights/{}/config", hub_url, API_KEY, self.event.light.value);
		let body: String = format!("{{\"startup\": {{\"customsettings\": {{{}}}}}}}", self.event.value);

		let put_client = reqwest::Client::new();
		let response = match put_client.put(&url).body(body).send().await
		{
			Ok(response) => response,
			Err(_) => return
		};

		// Check that request was successful
		(*self).is_activated = response.status() == reqwest::StatusCode::OK;
	}


	// pub fn make_request(&self) -> bool
	// {
	// 	return false;
	// }


	pub fn should_run(&self) -> bool
	{
		let current_timestamp: Timestamp = Local::now().timestamp();
		return self.timestamp <= current_timestamp;
	}
}
