import { Show, createEffect } from 'solid-js'

import './style/orders.scss'

import { createStore } from 'solid-js/store'
import { CountryDpy, ServiceDpy, httpx } from 'shared'
import { Order } from 'models'
import { useNavigate, useParams } from '@solidjs/router'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'

export default () => {
    type State = {
        orders: Order[]
        page: number
    }

    const [state, setState] = createStore<State>({ orders: [], page: 0 })
    const UP = useParams()
    const nav = useNavigate()

    createEffect(() => {
        let page = parseInt(UP.page || '0')
        if (isNaN(page) || page < 0) {
            nav('/orders/')
            return
        }
        fetch_orders(page)
    })

    function fetch_orders(page: number) {
        httpx({
            url: '/api/user/orders/?page=' + page,
            method: 'GET',
            type: 'json',
            onLoad(x) {
                if (x.status != 200) return

                setState({ orders: x.response, page })
            },
        })
    }

    const STATUS_TABLE: { [k in Order['status']]: string } = {
        done: 'تکمیل',
        wating: 'درحال تکمیل',
        refunded: 'بازپرداخت شد',
    }

    return (
        <div class='orders-fnd'>
            <Show when={state.orders.length == 0}>
                <div class='order-empty'>
                    <h2>سفارشی یافت نشد</h2>
                    <span>صفحه: {state.page}</span>
                </div>
            </Show>
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
            <div class='pagination'>
                <Show when={state.page > 0}>
                    <button
                        class='styled'
                        onClick={() => nav('/orders/' + (state.page - 1))}
                    >
                        <ChevronLeftIcon />
                    </button>
                </Show>
                <Show when={state.orders.length >= 32}>
                    <button
                        class='styled'
                        onClick={() => nav('/orders/' + (state.page + 1))}
                    >
                        <ChevronRightIcon />
                    </button>
                </Show>
            </div>
        </div>
    )
}
