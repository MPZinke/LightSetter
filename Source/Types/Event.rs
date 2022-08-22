
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


use crate::Types::Light::Light;


type Timestamp = i64;


#[derive(Clone, PartialEq)]
#[derive(Debug, serde::Serialize)]
pub struct Event
{
	pub id: i32,
	pub light: Light,
	pub hour: i32,
	pub minute: i32,
	pub path: String,
	pub value: String
}


impl Event
{
	pub fn new(row: &PgRow) -> Event
	{
		return Event{id: row.get("id"), hour: row.get("hour"), minute: row.get("minute"), path: row.get("path"),
		  value: row.get("value"), light: Light::new(&row)};
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
		if(todays_event_timestamp < Local::now().timestamp())
		{
			return todays_event_timestamp + 86_400;
		}
		return todays_event_timestamp;
	}


	/*
	SUMMARY: Gets the timestamp of the event for today, even if the time has passed.
	RETURNS: The timestamp of the event for current day.
	*/
	pub fn todays_timestamp(&self) -> i64
	{
		return Local::today().and_hms(self.hour as u32, self.minute as u32, 0).timestamp();
	}
}
