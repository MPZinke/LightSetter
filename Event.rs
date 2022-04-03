
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


pub enum Time
{
	DAYTIME,
	NIGHTTIME
}


impl Time
{
	pub fn copy(&self) -> Time
	{
		return match (self)
		{
		Time::DAYTIME => Time::DAYTIME,
		Time::NIGHTTIME => Time::NIGHTTIME
		};
	}
}


pub struct Event
{
	pub id: Time,
	pub hour: u32,
	pub minute: u32,
	pub poweron: &'static str
}


impl Event
{
	// ———————————————————————————————————————————————— CONSTRUCTORS ———————————————————————————————————————————————— //

	pub fn new(id: Time, hour: u32, minute: u32, poweron: &'static str) -> Event
	{
		return Event{id: id, hour: hour, minute: minute, poweron: poweron};
	}


	pub fn copy(&self) -> Event
	{
		return Event::new(self.id.copy(), self.hour, self.minute, self.poweron);
	}


	// ———————————————————————————————————————————————————— TIME ———————————————————————————————————————————————————— //

	pub fn is_active(&self) -> bool
	{
		let current_time: i64 = Local::now().timestamp();
		let event_time: i64 = Local::today().and_hms(self.hour, self.minute, 0).timestamp();

		return event_time < current_time;
	}


	pub fn timestamp(&self) -> i64
	{
		let current_time: i64 = Local::now().timestamp();
		let event_time: i64 = Local::today().and_hms(self.hour, self.minute, 0).timestamp();

		return event_time + ((event_time < current_time) as i64 * 8640);
	}
}
