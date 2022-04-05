
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


#[derive(PartialEq)]
pub enum LightID
{
	SINK
}


pub struct Light
{
	pub id: LightID,
	pub value: &'static str,
}
