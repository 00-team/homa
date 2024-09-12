import { CountryDpy, ServiceDpy } from 'shared'
import { Component, createEffect } from 'solid-js'
import { createStore } from 'solid-js/store'
import { httpx } from 'shared'
import { OrderStatus, PhoneOrder } from 'models'

const STATUS_TABLE: { [k in OrderStatus]: string } = {
    done: 'تکمیل',
    wating: 'درحال تکمیل',
    refunded: 'بازپرداخت شد',
}

type Props = {
    page: number
    update(loading: boolean, count: number): void
}
export const Phones: Component<Props> = P => {
    type State = {
        orders: PhoneOrder[]
    }

    const [state, setState] = createStore<State>({ orders: [] })

    createEffect(() => fetch_orders(P.page))

    function fetch_orders(page: number) {
        P.update(true, 0)
        httpx({
            url: '/api/user/orders/?page=' + page,
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
                        <span class='value datetime'>{o.datetime}</span>
                    </div>

                    <div class='row'>
                        <span class='key'>کشور:</span>
                        <span class='value'>
                            <CountryDpy d={o.country} />
                        </span>
                    </div>

                    <div class='row'>
                        <span class='key'>سرویس:</span>
                        <span class='value'>
                            <ServiceDpy d={o.service} />
                        </span>
                    </div>
                    <div class='row'>
                        <span class='key'>هزینه:</span>
                        <span class='value'>
                            {(~~(o.cost / 10)).toLocaleString()} تومان
                        </span>
                    </div>
                </div>
            ))}
        </div>
    )
}
