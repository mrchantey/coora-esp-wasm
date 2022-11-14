import { Response } from 'node-fetch'


export const parseResponse = async (prefix: string, prom: Promise<Response>, ip: string) => {
	const start = performance.now()
	const response = await prom
		.catch(_ => { throw new Error(`${prefix} - failed to connect\n\tip: ${ip}`) })
	if (response.status < 200 || response.status >= 300)
		throw new Error(`${prefix} - bad request: ${response.status}\n\tip: ${ip}`)
	const txt = await response.text()
	if (txt !== 'ok')
		throw new Error(`${prefix} - expected 'ok', received: ${txt}\n\tip: ${ip}`)
	const duration = performance.now() - start
	return { response, txt, duration }
}