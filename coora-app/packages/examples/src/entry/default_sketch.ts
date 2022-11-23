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
	// const t = math.sin(time.elapsed() as f32 / 1000) / 2 + 0.5
	// ledStrip.setLeds(0, t * 32 as u32, 0, 0)
	ledStrip.show()
}