
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.03                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use chrono::{DateTime, Date, Local};


use crate::Light::Light;
use crate::Event::Event;


pub struct LightEvent
{
	pub light: &'static Light,
	pub event: &'static Event,
	pub is_activated: bool,
	pub has_passed: bool,
	pub todays_time_stamp: i64
}


impl LightEvent
{
	pub fn new(light: &'static Light, event: &'static Event, is_activated: bool) -> LightEvent
	{
		let current_time: i64 = Local::now().timestamp();
		let event_time: i64 = Local::today().and_hms(event.hour, event.minute, 0).timestamp();

		return LightEvent
		{
			light: light,
			event: event,
			is_activated: false,
			has_passed: current_time < event_time,
			todays_time_stamp: event_time
		};
	}


	pub fn time_until(&self) -> i64
	{
		let current_time: i64 = Local::now().timestamp();

		if(self.todays_time_stamp < current_time)
		{
			return 0;
		}

		return self.todays_time_stamp - current_time;
	}
}
