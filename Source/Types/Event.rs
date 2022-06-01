
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


use chrono::{DateTime, Date, Local};
use sqlx::{postgres::PgRow, Row};


// use crate::Types::EventRequest::EventRequest;
use crate::Types::Light::Light;


type Time = i64;


#[derive(PartialEq)]
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
		return Event{id: row.get("id"), light: light, hour: row.get("hour"), minute: row.get("minute"),
		  value: row.get("value")};
	}


	// pub fn not_in_requests(&self, event_requests: &Vec<EventRequest>) -> bool
	// {
	// 	for event_request in event_requests.iter()
	// 	{
	// 		if(event_request.event == self)
	// 		{
	// 			return false;
	// 		}
	// 	}

	// 	return true;
	// }


	// // ———————————————————————————————————————————————————— TIME ———————————————————————————————————————————————————— //

	// /*
	// SUMMARY: Gets the time until the event for the current day.
	// DETAILS: Determines the current timestamp in seconds and the timestamp of the event.
	// RETURNS: The amount of time for the 
	// */
	// pub fn time_until_todays_event(&self) -> Time
	// {
	// 	let current_time: Time = Local::now().timestamp();
	// 	let timestamp: Time = self.timestamp();

	// 	return timestamp - current_time;
	// }


	
	// SUMMARY: Gets the timestamp of the event for today.
	// RETURNS: The timestamp of the event for today.
	
	// pub fn timestamp(&self) -> Time
	// {
	// 	return Local::today().and_hms(self.hour, self.minute, 0).timestamp();
	// }


	// /*
	// SUMMARY: Determines the time in seconds (Unix time) of the next event occurance.
	// DETAILS: Gets the current & event timestamp. Calculates the timestamp of the current day's event & adds a day of
	//          seconds to the timestamp if it is already passed (since the events are daily).
	// RETURNS: The time in seconds (Unix time) of the next event occurance.
	// */
	// pub fn next(&self) -> Time
	// {
	// 	let current_time: Time = Local::now().timestamp();
	// 	let event_time: Time = Local::today().and_hms(self.hour, self.minute, 0).timestamp();

	// 	return event_time + ((event_time < current_time) as Time * 8640);
	// }
}
