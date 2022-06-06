
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


use crate::LookupError::{LookupError, NewNotFoundError};
use crate::Types::Light::Light;


pub async fn SELECT_Light_by_id(pool: &PgPool, id: u32) -> Result<Light, LookupError>
{
	let query_str: &str = r#"
	  SELECT "id", "label", "value"
	  FROM "Light"
	  WHERE "id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;
	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Network`.`id`: '{}'", id)));
	}

	return Ok(Light::new(&result[0]));
}
