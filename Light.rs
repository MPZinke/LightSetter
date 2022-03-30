
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


use crate::Event::{Event, Time};


enum LightID
{
	SINK
}


pub struct Light<'a>
{
	pub id: LightID,
	pub value: &'a str,
	pub events: Vec<Event>
}


pub static LIGHTS: [Light; 1] =
[
	Light
	{
		id: LightID::SINK,
		value: "3",
		events:
		vec![
			Event{id: Time::DAYTIME, hour: 8, minute: 0, poweron: "\"ct\": 335"/* white */},
			Event{id: Time::NIGHTTIME, hour: 22, minute: 0, poweron: "\"xy\": [0.6867,0.3119]"/* red */}
		]
	}
];
