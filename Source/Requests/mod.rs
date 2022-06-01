
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.30                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use reqwest::header::AUTHORIZATION;


pub async fn get_hue_bridge_IP() -> String
{
	let mut headers = reqwest::header::HeaderMap::new();
	let auth_token: String = format!("Bearer {}", env!("NETWORKLOOKUP_BEARERTOKEN"));
	let auth_value = reqwest::header::HeaderValue::from_str(&auth_token).unwrap();
	headers.insert(AUTHORIZATION, auth_value);

	let client: reqwest::Client = reqwest::Client::builder().default_headers(headers).build().unwrap();
	let url: String = format!("http://localhost:8081/api/v1.0/network/label/{}/device/label/{}", "Home", "HueBridge");
	let body: String = client.get(url).send().await.unwrap().text().await.unwrap();

	let error_message: String = format!("Invalid response received: {}", body);
	let json_value: serde_json::Value = serde_json::from_str(&body).expect(&error_message);
	return json_value["address"].as_str().expect(&error_message).to_string();
}
