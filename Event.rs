
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


pub enum Time
{
	DAYTIME,
	NIGHTTIME
}


pub struct Event
{
	pub id: Time,
	pub hour: u32,
	pub minute: u32,
	pub poweron: &'static str
}
