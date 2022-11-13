import { core, led } from '../bindings'

//for testing
export function _millis():u64{
	return core.millis()
}

export function start():void{
	// core.log("howdy")
}

export function run():void{
	led.setAll(0, 32, 0, 0)
	led.show()
}
// export function add(a:i32,b:i32):i32{
// 	return a+b
// }