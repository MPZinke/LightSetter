
/***********************************************************************************************************************
*																													  *
*   created by: MPZinke																								*
*   on 2022.03.21																									  *
*																													  *
*   DESCRIPTION: TEMPLATE																							  *
*   BUGS:																											  *
*   FUTURE:																											*
*																													  *
***********************************************************************************************************************/


use chrono::{DateTime, Date, Local};


use crate::Event::{Event, Time};
use crate::LightEvent::LightEvent;


pub enum LightID
{
	SINK
}


// impl LightID
// {
// 	fn copy(&self) -> LightID
// 	{
// 		return match(self)
// 		{

// 		}
// 	}
// }


pub struct Light
{
	pub id: LightID,
	pub value: &'static str,
	pub events: [Event; 2]
}


impl Light
{
	pub fn next_event(&self) -> &Event
	{
		let mut next_event: &Event = &self.events[0];
		for x in 0..self.events.len()
		{
			if(self.events[x].timestamp() < next_event.timestamp())
			{
				next_event = &self.events[0];
			}
		}
		return next_event;
	}


	pub fn upcoming_events(&'static self) -> Vec<LightEvent>
	{
		let light_events: Vec<LightEvent> = vec![LightEvent::new(self, &self.events[0], false)];
		return light_events;
	}
}


pub static LIGHTS: [Light; 1] =
[
	Light
	{
		id: LightID::SINK,
		value: "3",
		events:
		[
			Event{id: Time::DAYTIME, hour: 8, minute: 0, poweron: "\"ct\": 335"/* white */},
			Event{id: Time::NIGHTTIME, hour: 22, minute: 0, poweron: "\"xy\": [0.6867,0.3119]"/* red */}
		]
	}
];



