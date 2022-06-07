
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


use serde_json;
use sqlx::{postgres::PgRow, Row};


static API_KEY: &str = env!("LIGHTSETTER_APIKEY");


#[derive(Clone, PartialEq)]
// #[derive(Debug, serde::Serialize)]
pub struct Light
{
	pub id: i32,
	pub label: String,
	pub value: String,
}


impl Light
{
	pub fn new(row: &PgRow) -> Light
	{
		return Light{id: row.get("Light.id"), label: row.get("Light.label"), value: row.get("Light.value")};
	}


	async fn request_light(&self, bridge_ip: &String) -> Option<serde_json::Value>
	{
		let url: String = format!("http://{}/api/{}/lights/{}", bridge_ip, API_KEY, self.value);
		let response = match(reqwest::get(&url).await)
		{
			Ok(response) => response,
			Err(err) => {println!("{}", err); return None}
		};

		let response_body: String = match(response.text().await)
		{
			Ok(response_body) => response_body,
			Err(_) => return None
		};

		match(serde_json::from_str(&response_body))
		{
			Ok(json_object) => return Some(json_object),
			Err(err) => {println!("{}", err); return None}
		}
	}


	pub async fn is_reachable(&self, bridge_ip: &String) -> bool
	{
		let json_object: serde_json::Value = match(self.request_light(bridge_ip).await)
		{
			Some(json_object) => json_object,
			None => return false
		};

		let state: serde_json::Value = match(json_object.get("state"))
		{
			Some(state) => state.clone(),
			None => return false
		};

		let reachable: serde_json::Value = match(state.get("reachable"))
		{
			Some(reachable) => reachable.clone(),
			None => return false
		};

		return reachable.as_bool().unwrap_or(false);
	}
	// // SUMMARY: Determines if the light is able to receive a request.
	// // PARAMS:  Takes the ID of the light to query.
	// // DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is
	// //          on and reachable.
	// // RETURNS: Whether the light is reachable.
	// pub fn light_is_reachable(light_id: &str) -> bool
	// {
	// }


	// // SUMMARY: Gets the JSON parsed info for a light.
	// // PARAMS:  Takes the ID of the light to query.
	// // DETAILS: Makes an HTTP request to the Hub for the light to get the light info. Parses GET request body into a
	// //          JsonValue.
	// // RETURNS: Ok(A parsed JsonValue of the light info) or Err(String of the error message).
	// pub fn light_info(light_id: &str) -> Result<JsonValue, String>
	// {
	// }
}
