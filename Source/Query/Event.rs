
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


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::LookupError::LookupError;
use crate::Query::Light::SELECT_Light_by_id;
use crate::Types::{Event::Event, Light::Light};


pub async fn SELECT_Events(pool: &PgPool) -> Result<Vec<Event>, LookupError>
{
	let query_str: &str = r#"
	  SELECT "id", "label", "hour", "minute", "value"
	  "Light"."id" AS "Light.id", "Light"."label" AS "Light.label", "Light"."value" AS "Light.value"
	  FROM "Events"
	  JOIN "Light" ON "Event"."Light.id" = "Light"."id";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut events: Vec<Event> = vec![];
	for row in result
	{
		events.push(Event::new(&row, Light{id: row.get("id"), label: row.get("label"), value: row.get("value")}));
	}
	return Ok(events);
}
