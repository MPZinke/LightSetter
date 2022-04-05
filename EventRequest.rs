
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


	pub fn time_remaining(&self) -> i64
	{
		let current_time: Time = Local::now().timestamp();
		return self.timestamp - current_time;
	}
}
