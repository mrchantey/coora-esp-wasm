# Plan

## Mission

How might I simplify embedded development for all creators so that their positive energy is amplified.

## MVS

Enable embedded development wirelessly from a browser.

## Risks & Blocks

- Coora is unsolvable by us
	- **Impact:** The companly does not create value
	- **Mitigate:** Rapidly prototype a MVS
- Coora creates a closed ecosystem
	- **Impact:** Users would prefer a more difficult but open one
	- **Mitigate:** Only monetize the top layer of the stack, and open source the engine
- Coora remains undiscovered
	- **Impact:** Even if we solve the problem, nobody will know about it
	- **Mitigate:** Work with local communities and run engagement workshops as soon as possible
- Makers are already used to arduino
	- **Impact:** Even if this is better, change is hard
	- **Mitigate:**
		1. Make onboarding as easy as possible
		2. target different use case
		3. Encourage experimentation over change 'have a little try'
- Devs don't want yet another standard
	- **Impact:** Developers will not create plugins
	- **Mitigate:** Ensure the solution is at least 10x better
- Educators like polished and safe systems
	- **Impact:** Educators will not adopt the product
	- **Mitigate:** Use 'Wizard' brand archetype to paint company as magical

## Stakeholders

- Makers
	- **Role:** End User & Advocate
	- **Arrangement:** They use the tech because it enables their creativity
	- **Engagement:** Through workshops
- Developers
	- **Role:** Plugin developers
	- **Arrangement:** They want to extend the platform
	- **Engagement:** Engage through Social Media
- Educators
	- **Role:** End User & Advocate
	- **Arrangement:** They are excited by the tech because it makes their jobs easier
	- **Engagement:** Local communities

## Roadmap

### 0.0.1
- Engine
	- Wasm Runtime
	- Wifi config
	- HTTP Server
	- Plugins
		- Core
		- Led
- App
	- npm template
		- Assemblyscript hello led world
	- npm cli
		- Flash & Configure, Build & Deploy
- Docs
	- Branding website

### 0.0.2
- Engine
	- plugin bindings generators
		- export typescript definitions
		- export rust wasm definitions
		- export wasm glue code

### 0.0.3
- Targets
	- ESP32
	- Emulator