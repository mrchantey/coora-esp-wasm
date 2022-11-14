import fetch from 'node-fetch'

export const ping = async (url: string) => {
	//we need to make this a get
	const response = await fetch(url, { method: 'POST' })
		.catch(_ => { throw new Error('ping timeout') })
	if (response.status < 200 || response.status >= 300)
		throw new Error(`bad request: ${response.status}`)
	const txt = await response.text()
	if (txt !== 'ok')
		throw new Error(`expected 'ok', received: ${txt}`)
	return true
}
// export const ping = async (val: string) => 'chicken'