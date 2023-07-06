# MQTT

## Quickstart
1. `docker run -d --name emqx -p 1883:1883 -p 8083:8083 -p 8084:8084 -p 8883:8883 -p 18083:18083 emqx/emqx:5.0.10`
2. [Visit dashboard](http://localhost:18083)
3. [Open test client](http://localhost:18083/#/websocket)


## Ports
- Nonsecure: MQTT `1883`
- Secure: `8883`

## Setup

1. You will need docker
	- windows: `choco install docker-desktop`
2. Get EMQX Image
	- https://www.emqx.com/en/try?product=broker
	- See download page for up to date documentation
	- Start image using documentation
3. [quickstart](#quickstart)

## Resources
- [Javascript Client](https://www.emqx.io/docs/en/v5.0/development/javascript.html#mqtt-js-usage-example)
- [ESP32 Client](https://github.com/emqx/MQTT-Client-Examples/blob/master/mqtt-client-ESP32/esp32_connect_mqtt.ino.ino)



