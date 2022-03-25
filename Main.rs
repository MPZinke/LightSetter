
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


use json::{JsonValue, parse};
use reqwest;


static HUB_URL: &str = env!("HUB_URL");
static API_KEY: &str = env!("API_KEY");

static LIGHT_NUMBER: &str = "3";
static DAY_TIME_VALUE: &str = "\"ct\": 335";  // White
static NIGHT_TIME_VALUE: &str = "\"xy\": [0.6867,0.3119]";  // Red


fn attribute_is_true(mut json_string: JsonValue, attribute: &str) -> bool
{
	return json_string.remove(attribute).as_bool().unwrap_or(false);
}


// SUMMARY: Determines if the light is able to receive a request.
// DETAILS: Makes a HTTP GET request to get the light's info. Reads the response JSON and determines if the light is on
//          and reachable.
// RETURNS: Whether the light is reachable.
fn light_is_reachable(light_id: &str) -> bool
{
	let mut light_info_object: JsonValue = match light_info(light_id)
	{
		Ok(mut light_info_object) => light_info_object,
		Err(_) => return false
	};

	return light_info_object.remove("state").remove("reachable").as_bool().unwrap_or(false);
}




fn light_info(light_id: &str) -> Result<JsonValue, String>
{
	let url: String = format!("http://{}/api/{}/lights/{}", HUB_URL, API_KEY, light_id);

	let mut response = match reqwest::get(&url)
	{
		Ok(mut response) => response,
		Err(err) => return Err(err.to_string())
	};

	// Check that request was successful
	if(response.status() != reqwest::StatusCode::Ok)
	{
		return Err(format!("HTTP Status Code {} received", response.status()));
	}

	// Set body
	let response_body: String = match response.text()
	{
		Ok(response_body) => response_body,
		Err(err) => return Err(format!("No response body found for get request to {}", response.url()))
	};

	println!("{}", response_body);

	return match parse(&response_body)
	{
		Ok(json_object) => Ok(json_object),
		Err(err) => Err(err.to_string())
	};
}


// SUMMARY: Makes a request to the Hub to configure the light.
// PARAMS:  Takes the value to configure the light color to.
// DETAILS: Makes an HTTP PUT request to the light to set its color.
// RETURNS: Whether the PUT request returns a 200 status
fn set_poweron_color(light_id: &str, poweron_color: &str) -> bool
{
	let url: String = format!("http://{}/api/{}/lights/{}/config", HUB_URL, API_KEY, light_id);
	let body: String = format!("{{\"startup\": {{\"customsettings\": {{{}}}}}}}", poweron_color);

	let put_client = reqwest::Client::new();
	let response = match put_client.put(&url).body(body).send()
	{
		Ok(response) => response,
		Err(_) => return false
	};

	// Check that request was successful
	return response.status() == reqwest::StatusCode::Ok;
}


fn sleep_time() -> u32
{
	return 1 as u32;
}



fn main()
{
	while(true)
	{
		if(light_is_reachable(LIGHT_NUMBER))
		{
			println!("Light is reachable");
			if(set_poweron_color(LIGHT_NUMBER, DAY_TIME_VALUE))
			{
				println!("Light value set");
			}
			else
			{
				println!("Failed to set light value");
			}
		}
		else
		{
			println!("False");
		}
		break;
	}
}