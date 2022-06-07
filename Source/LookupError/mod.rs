
/***********************************************************************************************************************
*																													  *
*   created by: MPZinke																								*
*   on 2022.05.24																									  *
*																													  *
*   DESCRIPTION: TEMPLATE																							  *
*   BUGS:																											  *
*   FUTURE:																											*
*																													  *
***********************************************************************************************************************/


// ——————————————————————————————————————————————————— ERROR ENUM ——————————————————————————————————————————————————— //

// FROM: https://fettblog.eu/rust-enums-wrapping-errors/
//  AND: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
#[derive(Debug)]
pub enum LookupError
{
	InvalidHeader(reqwest::header::InvalidHeaderValue),
	NotFound(std::io::Error),
	Postgres(sqlx::error::Error),
	Request(reqwest::Error)
}


impl std::fmt::Display for LookupError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			LookupError::InvalidHeader(error) => write!(format, "{}", error),
			LookupError::NotFound(error) => write!(format, "{}", error),
			LookupError::Postgres(error) => write!(format, "{}", error),
			LookupError::Request(error) => write!(format, "{}", error)
		}
	}
}


impl From<reqwest::header::InvalidHeaderValue> for LookupError
{
	fn from(err: reqwest::header::InvalidHeaderValue) -> Self
	{
		LookupError::InvalidHeader(err)
	}
}


impl From<std::io::Error> for LookupError
{
	fn from(err: std::io::Error) -> Self
	{
		LookupError::NotFound(err)
	}
}


impl From<sqlx::error::Error> for LookupError
{
	fn from(err: sqlx::error::Error) -> Self
	{
		LookupError::Postgres(err)
	}
}


impl From<reqwest::Error> for LookupError
{
	fn from(err: reqwest::Error) -> Self
	{
		LookupError::Request(err)
	}
}


// // ———————————————————————————————————————————————— HELPER FUNCTIONS ———————————————————————————————————————————————— //

// pub fn NewNotFoundError(message: String) -> LookupError
// {
// 	return LookupError::NotFound(std::io::Error::new(std::io::ErrorKind::NotFound, message));
// }
