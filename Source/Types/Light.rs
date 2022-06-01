
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
use sqlx::{postgres::PgRow, Row};


#[derive(PartialEq)]
pub struct Light
{
	pub id: u32,
	pub label: String,
	pub value: String,
}


impl Light
{
	pub fn new(row: &PgRow) -> Light
	{
		return Light{id: row.get("id"), label: row.get("label"), value: row.get("value")};
	}
	// // SUMMARY: Determines if the light is able to receive a request.
	// // PARAMS:  Takes the ID of the light to query.
	// // DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is
	// //          on and reachable.
	// // RETURNS: Whether the light is reachable.
	// pub fn light_is_reachable(light_id: &str) -> bool
	// {
	// 	let mut light_info_object: JsonValue = match self.light_info(light_id)
	// 	{
	// 		Ok(mut light_info_object) => light_info_object,
	// 		Err(_) => return false
	// 	};

	// 	return light_info_object.remove("state").remove("reachable").as_bool().unwrap_or(false);
	// }


	// // SUMMARY: Gets the JSON parsed info for a light.
	// // PARAMS:  Takes the ID of the light to query.
	// // DETAILS: Makes an HTTP request to the Hub for the light to get the light info. Parses GET request body into a
	// //          JsonValue.
	// // RETURNS: Ok(A parsed JsonValue of the light info) or Err(String of the error message).
	// pub fn light_info(light_id: &str) -> Result<JsonValue, String>
	// {
	// 	let url: String = format!("http://{}/api/{}/lights/{}", HUB_URL, API_KEY, light_id);

	// 	let mut response = match reqwest::get(&url)
	// 	{
	// 		Ok(mut response) => response,
	// 		Err(err) => return Err(err.to_string())
	// 	};

	// 	// Check that request was successful
	// 	if(response.status() != reqwest::StatusCode::Ok)
	// 	{
	// 		return Err(format!("HTTP Status Code {} received", response.status()));
	// 	}

	// 	// Set body
	// 	let response_body: String = match response.text()
	// 	{
	// 		Ok(response_body) => response_body,
	// 		Err(err) => return Err(format!("No response body found for get request to {}", response.url()))
	// 	};

	// 	println!("{}", response_body);

	// 	return match parse(&response_body)
	// 	{
	// 		Ok(json_object) => Ok(json_object),
	// 		Err(err) => Err(err.to_string())
	// 	};
	// }
}
