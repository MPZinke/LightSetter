
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


#![allow(non_snake_case)]
#![allow(unused_parens)]


mod Light;


use json;
use reqwest;


static HUB_URL: &str = env!("HUB_URL");
static API_KEY: &str = env!("API_KEY");

static LIGHT_NUMBER: &str = "3";
static DAY_TIME_VALUE: &str = "\"ct\": 335";  // White
static NIGHT_TIME_VALUE: &str = "\"xy\": [0.6867,0.3119]";  // Red


fn is_on_and_is_reachable(light_json: String) -> bool
{
	let mut light = match json::parse(&light_json)
	{
		Ok(mut light) => light,
		Err(_) => return false
	};

	let mut light_state = light.remove("state");

	let is_on: bool = light_state.remove("on").as_bool().unwrap_or(false);
	let is_reachable: bool = light_state.remove("reachable").as_bool().unwrap_or(false);

	return is_on && is_reachable;
}


// SUMMARY: Determines if the light is on to receive a request.
// DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is on
//          and reachable.
// RETURNS: Whether the light is on and reachable.
fn light_is_on() -> bool
{
	let url: String = format!("http://{}/api/{}/lights/{}", HUB_URL, API_KEY, LIGHT_NUMBER);

	let mut response = match reqwest::get(&url)
	{
		Ok(mut response) => response,
		Err(_) => return false
	};

	if(response.status() != reqwest::StatusCode::Ok)
	{
		return false;
	}

	let light_json: String = match response.text()
	{
		Ok(light_json) => light_json,
		Err(_) => return false
	};

	return is_on_and_is_reachable(light_json);
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
		if(light_is_on())
		{
			println!("True");
		}
		else
		{
			println!("False");
		}
		break;
	}
}