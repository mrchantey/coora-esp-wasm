// memory.

export function size(): u32{
	return memory.size()
}

export function _load(ptr: i32): i32{
	// const a = 2
	return load<i32>(ptr)
}

export function _store(ptr: i32, val: i32): void{
	return store<i32>(ptr, val)
}

export function add(a: i32, b: i32): i32{
	return a + b
}


const a: string = '\1\2\3\4'