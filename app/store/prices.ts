import { httpx } from 'shared'
import { createEffect, createRoot, on } from 'solid-js'
import { createStore } from 'solid-js/store'

type State = {
    update: number
    data: { [k: string]: [number, number] }
}

const [prices, setPrices] = createStore<State>({ update: 0, data: {} })

createRoot(() => {
    createEffect(
        on(
            () => {
                prices.update
            },
            () => {
                if (prices.update + 600e3 > Date.now()) return

                httpx({
                    method: 'GET',
                    url: '/api/vendor/check-service/',
                    onLoad(x) {
                        if (x.status != 200) return

                        setPrices({
                            update: Date.now(),
                            data: x.response,
                        })
                    },
                })
            },
            { defer: true }
        )
    )
})

export { prices }
