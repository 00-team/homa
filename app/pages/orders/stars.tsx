import { Component, Show, createEffect } from 'solid-js'
import { createStore } from 'solid-js/store'
import { STATUS_TABLE, TomanDpy, fmt_timestamp, httpx } from 'shared'
import { StarOrder } from 'models'

type Props = {
    page: number
    update(loading: boolean, count: number): void
}
export const Stars: Component<Props> = P => {
    type State = {
        orders: StarOrder[]
    }

    const [state, setState] = createStore<State>({
        orders: [],
    })

    createEffect(() => fetch_orders(P.page))

    function fetch_orders(page: number) {
        P.update(true, 0)
        httpx({
            url: '/api/user/star-orders/?page=' + page,
            method: 'GET',
            type: 'json',
            onLoad(x) {
                if (x.status != 200) {
                    P.update(false, 0)
                    return
                }
                setState({ orders: x.response })
                P.update(false, x.response.length)
            },
        })
    }

    return (
        <div class='order-list'>
            {state.orders.map(o => (
                <div class='order'>
                    <div class='row'>
                        <span class='key'>وضعیت:</span>
                        <span class='value'>{STATUS_TABLE[o.status]}</span>
                    </div>
                    <div class='row'>
                        <span class='key'>تاریخ:</span>
                        <span class='value datetime'>
                            {fmt_timestamp(o.timestamp)}
                        </span>
                    </div>
                    <div class='row'>
                        <span class='key'>مقدار:</span>
                        <span class='value'>{o.amount} استار</span>
                    </div>
                    <div class='row'>
                        <span class='key'>قیمت:</span>
                        <span class='value'>
                            <TomanDpy irr={o.cost} /> تومان
                        </span>
                    </div>
                    <Show when={o.status == 'done'}>
                        <div class='row'>
                            <span class='key'>هش:</span>
                            <span class='value'>{o.hash}</span>
                        </div>
                    </Show>
                </div>
            ))}
        </div>
    )
}
