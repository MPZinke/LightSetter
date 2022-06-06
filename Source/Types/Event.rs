
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.03.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use chrono::Local;
use sqlx::{postgres::PgRow, Row};


use crate::Types::EventRequest::EventRequest;
use crate::Types::Light::Light;


type Timestamp = i64;


#[derive(Clone, PartialEq)]
pub struct Event
{
	pub id: u32,
	pub light: Light,
	pub hour: u32,
	pub minute: u32,
	pub value: String
}


impl Event
{
	pub fn new(row: &PgRow, light: Light) -> Event
	{
		return Event{id: row.get("id"), light: light.clone(), hour: row.get("hour"), minute: row.get("minute"),
		  value: row.get("value")};
	}


	// ———————————————————————————————————————————————————— TIME ———————————————————————————————————————————————————— //

	/*
	SUMMARY: Determines the time of the next event.
	DETAILS: Compares the time of the event for today with the current timestamp. If the event's time is less than or
	         equal to the current time then the event is considered passed for the day, and will return tomorrow's
	         event's time.
	RETURNS: The timestamp of today's event if it has not passed, otherwise tomorrow's timestamp.
	*/
	pub fn next_timestamp(&self) -> i64
	{
		let todays_event_timestamp: Timestamp = self.todays_timestamp();
		if(todays_event_timestamp <= Local::now().timestamp())
		{
			return todays_event_timestamp;
		}
		return todays_event_timestamp + 86_400;
	}


	/*
	SUMMARY: Gets the timestamp of the event for today, even if the time has passed.
	RETURNS: The timestamp of the event for current day.
	*/
	pub fn todays_timestamp(&self) -> i64
	{
		return Local::today().and_hms(self.hour, self.minute, 0).timestamp();
	}


	/*
	SUMMARY: Gets the timestamp of the event for tomorrow.
	DETAILS: Calculates the timestamp of today's event and adds 86,400 seconds to it.
	RETURNS: The timestamp of the event for tomorrow.
	NOTES:   The timestamp might be off due to events like daylights savings time.
	*/
	pub fn tomorrows_timestamp(&self) -> i64
	{
		return Local::today().and_hms(self.hour, self.minute, 0).timestamp() + 86_400;
	}


	pub fn is_in_requests(&self, event_requests: &Vec<EventRequest>) -> bool
	{
		for event_request in event_requests.iter()
		{
			if(*self == event_request.event)
			{
				return true;
			}
		}
		return false;
	}

	// /*
	// SUMMARY: Gets the time until the event for the current day.
	// DETAILS: Determines the current timestamp in seconds and the timestamp of the event.
	// RETURNS: The amount of time for the 
	// */
	// pub fn time_until_todays_event(&self) -> Timestamp
	// {
	// 	let current_time: Timestamp = Local::now().timestamp();
	// 	let timestamp: Timestamp = self.timestamp();

	// 	return timestamp - current_time;
	// }


	
	
	// pub fn timestamp(&self) -> Timestamp
	// {
	// }


	// /*
	// SUMMARY: Determines the time in seconds (Unix time) of the next event occurance.
	// DETAILS: Gets the current & event timestamp. Calculates the timestamp of the current day's event & adds a day of
	//          seconds to the timestamp if it is already passed (since the events are daily).
	// RETURNS: The time in seconds (Unix time) of the next event occurance.
	// */
	// pub fn next(&self) -> Timestamp
	// {
	// 	let current_time: Timestamp = Local::now().timestamp();
	// 	let event_time: Timestamp = Local::today().and_hms(self.hour, self.minute, 0).timestamp();

	// 	return event_time + ((event_time < current_time) as Timestamp * 8640);
	// }
}
