
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


use crate::Types::Event::Event;


type Timestamp = i64;


static API_KEY: &str = env!("LIGHTSETTER_HUE_APIKEY");


#[derive(Clone, PartialEq)]
#[derive(Debug, serde::Serialize)]
pub struct EventRequest
{
	pub event: Event,
	pub is_activated: bool,  // whether the request has been activated
	pub timestamp: i64  // timestamp of this request's activation
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


	pub fn is_not_superseded_by_more_recent_light_event(&self, event_requests: &Vec<EventRequest>) -> bool
	{
		let current_timestamp: Timestamp = Local::now().timestamp();
		if(current_timestamp <= self.timestamp)  // if request has not yet occurred/run
		{
			return true;
		}

		for x in 0..event_requests.len()
		{
			// If the request's event's light's id == a different request's event's light's id
			if(event_requests[x].event.light.id == self.event.light.id && event_requests[x] != *self
			  && self.timestamp < event_requests[x].timestamp && event_requests[x].timestamp <= current_timestamp)
			{
				return false;
			}
		}

		return true;
	}


	// ——————————————————————————————————————————————————— OTHER  ——————————————————————————————————————————————————— //

	pub fn print(&self) -> ()
	{
		println!("Setting light {} to {}", self.event.light.label, self.event.value);
	}


	// —————————————————————————————————————————————————— REQUESTS —————————————————————————————————————————————————— //

	/*
	SUMMARY: Makes a request to the Hub to configure the light.
	PARAMS:  Takes the value to configure the light color to.
	DETAILS: Makes an HTTP PUT request to the light to set its color.
	RETURNS: Whether the PUT request returns a 200 status
	*/
	pub async fn run(&mut self, bridge_ip: &String) -> ()
	{
		self.print();  //TESTING
		if(!self.event.light.is_reachable(bridge_ip).await)
		{
			return;
		}

		let url: String = format!("http://{}/api/{}/lights/{}/{}", bridge_ip, API_KEY, self.event.light.value,
		  self.event.path);

		let put_client = reqwest::Client::new();
		let response = match put_client.put(&url).body(self.event.value.clone()).send().await
		{
			Ok(response) => response,
			Err(_) => return
		};

		// Check that request was successful
		(*self).is_activated = response.status() == reqwest::StatusCode::OK;
	}


	pub fn should_run(&self) -> bool
	{
		let current_timestamp: Timestamp = Local::now().timestamp();
		return self.timestamp <= current_timestamp;
	}
}
