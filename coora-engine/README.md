# Coora Engine
A rust program run on microcontrollers and allows the uploading and running of Coora Apps 

## Targets



## Plugins

Currently the plugins are located in `crates/engine/src/plugins` but the idea is they will eventually be put in their own crates.
I'm thinking a naming convention something like `coora_plugin_*plugin_name*`.


- Current
	- Core
	- Led
		- SK6812 - RGBW
			- `coora_plugin_led_sk6812rgbw`
- Planned
	- IMU
	- Motor
