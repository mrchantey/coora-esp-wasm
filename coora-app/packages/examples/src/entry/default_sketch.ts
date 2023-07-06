import { debug, ledStrip, time } from '../bindings'
//for testing
export function ping(): u64{
	return time.elapsed()
}

export function start(): void{
	debug.log('🤘running default sketch🤘')
	ledStrip.setLeds(0, 4, 6, 0)
	ledStrip.show()
}

export function update(): void{	
	ledStrip.show()
}