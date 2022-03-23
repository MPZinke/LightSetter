
/***********************************************************************************************************************
*																													  *
*   created by: MPZinke																								*
*   on 2022.03.21																									  *
*																													  *
*   DESCRIPTION: TEMPLATE																							  *
*   BUGS:																											  *
*   FUTURE:																											*
*																													  *
***********************************************************************************************************************/


pub mod Shared
{
	pub mod Capabilities
	{
		pub struct Streaming
		{
			proxy: bool,
			renderer: bool
		}
	}


	pub struct Swupdate
	{
		pub lastinstall: String,
		pub state: String
	}
}


pub mod Color
{
	use crate::Light::Shared::Swupdate;


	pub mod Capabilities
	{
		pub mod Control
		{
			use crate::Light::Shared::Capabilities::Streaming;


			pub struct Control
			{
				colorgamut: [[f32; 2]; 3],
				colorgamuttype: String,
				ct: Ct,
				maxlumen: u16,
				mindimlevel: u16
			}


			pub struct Ct
			{
				max: u16,
				min: u16,
			}
		}


		pub struct Capabilities
		{
			certified: bool,
			control: Control::Control,
			streaming: crate::Light::Shared::Capabilities::Streaming
		}
	}


	pub mod Config
	{
		pub mod Startup
		{
			pub struct Startup
			{
				configured: bool,
				customsettings: Customsettings,
				mode: String
			}


			pub struct Customsettings
			{
				bri: u8,
				xy: [f32; 2]
			}
		}


		pub struct Config
		{
			archetype: String,
			direction: String,
			function: String,
			startup: Startup::Startup
		}
	}


	pub struct Light
	{
		pub capabilities: Capabilities::Capabilities,
		pub config: Config::Config,
		pub manufacturername: String,
		pub modelid: String,
		pub name: String,
		pub productid: String,
		pub productname: String,
		pub state: State,
		pub swconfigid: String,
		pub swupdate: Swupdate,
		pub swversion: String,
		pub r#type: String,
		pub uniqueid: String
	}


	pub struct State
	{
		pub alert: String,
		pub bri: u8,
		pub colormode: String,
		pub ct: u16,
		pub effect: String,
		pub hue: u16,
		pub mode: String,
		pub on: bool,
		pub reachable: bool,
		pub sat: u8,
		pub xy: [f32; 2]
	}



}


pub mod White
{
	use super::Shared::Swupdate;


	pub mod Capabilities
	{
		use crate::Light::Shared::Capabilities::Streaming;


		pub struct Capabilities
		{
			certified: bool,
			control: Control,
			streaming: crate::Light::Shared::Capabilities::Streaming
		}


		pub struct Control
		{
			maxlumen: u16,
			mindimlevel: u16
		}
	}


	pub mod Config
	{
        pub mod Startup
        {
        	pub struct Startup
        	{
        		configured: bool,
        		customsettings: Customsettings,
        		mode: String,
        	}


        	pub struct Customsettings
        	{
        		bri: u8
        	}
        }


    	pub struct Config
    	{
    		archetype: String,
			direction: String,
			function: String,
			startup: Startup::Startup
    	}
	}


	pub struct Light
	{
		capabilities: Capabilities::Capabilities,
		config: Config::Config,
		manufacturername: String,
		modelid: String,
		name: String,
		productid: String,
		productname: String,
		state: State,
		swconfigid: String,
		swupdate: Swupdate,
		swversion: String,
		r#type: String,
		uniqueid: String
	}


	pub struct State
	{
		pub alert: String,
		pub bri: u8,
		pub mode: String,
		pub on: bool,
		pub reachable: bool
	}



}




