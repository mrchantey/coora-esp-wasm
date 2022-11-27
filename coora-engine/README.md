# Coora Engine
A rust program run on microcontrollers and allows the uploading and running of Coora Apps 

## Targets

``

## Plugins

Currently the plugins are located in `crates/engine/src/plugins` but the idea is they will eventually be put in their own crates.
I'm thinking a naming convention something like .

- Current
	- Core
	- Led
		- SK6812 - RGBW
			- `coora_plugin_led_sk6812rgbw`
- Planned
	- IMU
	- Motor


### Naming Conventions

For crates both inside this repo and elsewhere the naming conventions should be as follows:
- Targets
	- `coora_target_*target_name*`
	- ie 
		- `coora_target_stm32`
- Plugins
	- `coora_plugin_*plugin_name*`
	- ie 
		- `coora_plugin_led_display`
- Target Plugins
	- For custom plugins their implementations should exist in individual crates:
	- `coora_plugin_*plugin_name*_*target_name*`
	- ie 
		- `coora_plugin_led_display_esp32`
		- `coora_plugin_led_display_stm32`


## Tradeoffs

- Non generic user state
	- In making the userstate generic, basically the app and every plugin would also need to be. Side effects of this mean being unable to collect plugins etc, so instead userstate is a heap allocated key-value store.