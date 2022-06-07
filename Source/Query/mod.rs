
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::PgPool;


pub mod Event;
pub mod Light;


pub async fn new_connection_pool() -> PgPool
{
	let host: &str = "localhost";
	// let user: &str = "root";
	let user: &str = "mpzinke";
	let DB_name: &str = "LightSetter";

	let connection_str: String = format!("postgres://{}@{}:5432/{}", user, host, DB_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str).await
	  .expect("Failed to create Postgres DB connection pool");

	return connection_pool;
}
