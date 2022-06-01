
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


use crate::Light::{Light, LightID};
use crate::Event::Event;


pub static LIGHTS: [Light; 1] =
[
	Light{id: LightID::SINK, value: "3"}
];


pub static EVENTS: [Event; 2] =
[
	Event{light: LightID::SINK, hour: 8, minute: 0, poweron: "\"ct\": 335"/* white */},
	Event{light: LightID::SINK, hour: 22, minute: 0, poweron: "\"xy\": [0.6867,0.3119]"/* red */}
];
