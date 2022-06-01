
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


pub mod Event;
pub mod Light;


use serde_json;
use serde::Serialize;


use crate::LookupError::LookupError;


/*
SUMMARY: Determines whether the query is a NotFound LookupError.
PARAMS:  Takes the generic Response.
DETAILS: Unwraps the Result & LookupError if Result is an LookupError.
RETURNS: If the unwrapped LookupError is NotFound, returns True, otherwise False.
*/
pub fn query_NotFound<T: Serialize>(generic_query: &Result<T, LookupError>) -> bool
{
	match(generic_query)
	{
		Ok(_) => return false,
		Err(error)
		=>
		{
			match(error)
			{
				LookupError::NotFound(_) => return true,
				LookupError::Generic(_) => return false,
				LookupError::InvalidHeader(_) => return false,
				LookupError::Postgres(_) => return false,
				LookupError::Request(_) => return false
			}
		}
	}
}
