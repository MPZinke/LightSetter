
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


use sqlx::{query, PgPool, postgres::PgRow};


use crate::LookupError::LookupError;
use crate::Types::Event::Event;


pub async fn SELECT_Events(pool: &PgPool) -> Result<Vec<Event>, LookupError>
{
	let query_str: &str = r#"
	  SELECT "Event"."id", "Event"."label", "hour", "minute", path, "Event"."value",
	  "Light"."id" AS "Light.id", "Light"."label" AS "Light.label", "Light"."value" AS "Light.value"
	  FROM "Event"
	  JOIN "Light" ON "Event"."Light.id" = "Light"."id";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut events: Vec<Event> = vec![];
	for row in result
	{
		// events.push(Event::new(&row, Light{id: row.get("id"), label: row.get("label"), value: row.get("value")}));
		events.push(Event::new(&row));
	}
	return Ok(events);
}
