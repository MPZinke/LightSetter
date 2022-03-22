
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.03.20                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


mod Light;


// use http::{request::Builder, request::Request, Response, Result};
use json;
use reqwest;
// use serde::de;


static HUB_URL: &str = env!("HUB_URL");
static API_KEY: &str = env!("API_KEY");

static LIGHT_NUMBER: &str = "3";
static DAY_TIME_VALUE: &str = "\"ct\": 335";  // White
static NIGHT_TIME_VALUE: &str = "\"xy\": [0.6867,0.3119]";  // Red


// SUMMARY: Determines if the light is on to receive a request.
// DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is on
//          and reachable.
// RETURNS: Whether the light is on and reachable.
fn light_is_on() -> bool
{
	let url: String = format!("http://{}/api/{}/lights/{}", HUB_URL, API_KEY, LIGHT_NUMBER);

	match reqwest::get(&url)
	{
		Ok(mut response)
		=>
		{
			if(response.status() != reqwest::StatusCode::Ok)
			{
				return false;
			}

			let light_json: Light::Light = match response.json()
			{
				Ok(light_json) => light_json,
				Err(_) => return false
			};
			return light_json.state.on && light_json.state.reachable;
		}
		Err(_)
		=> return false;
	}
}


// SUMMARY: Makes a request to the Hub to configure the light.
// PARAMS:  Takes the value to configure the light color to.
// DETAILS: Makes an HTTP PUT request to the light to set its color.
// RETURNS: 
fn set_poweron_color(poweron_color: String) -> bool
{
	let url: String = format!("http://{}/api/{}/lights/{}/config", HUB_URL, API_KEY, LIGHT_NUMBER);
	let body: String = format!("{{\"startup\": {{\"customsettings\": {}}}}}", poweron_color);
	// let request = Request::put(url).body(body);
	return false;
}


fn sleep_time() -> u32
{
	return 1 as u32;
}



fn main()
{
	while(true)
	{

		break;
	}
}