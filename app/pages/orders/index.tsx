import { Component, Show, createEffect } from 'solid-js'
import { createStore } from 'solid-js/store'
import { TomanDpy, fmt_timestamp, httpx } from 'shared'
import { OrderStatus, PhoneOrder, StarOrder } from 'models'
import { useNavigate, useParams } from '@solidjs/router'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'

import './style/index.scss'

const STATUS_TABLE: { [k in OrderStatus]: string } = {
    done: 'تکمیل',
    wating: 'درحال تکمیل',
    refunded: 'بازپرداخت شد',
}

export default () => {
    type State = {
        tab: 'star' | 'phone'
        page: number
        count: number
        loading: boolean
    }
    const [state, setState] = createStore<State>({
        tab: 'star',
        page: 0,
        count: 0,
        loading: true,
    })
    const UP = useParams()
    const nav = useNavigate()

    createEffect(() => {
        let page = parseInt(UP.page || '0')
        if (isNaN(page) || page < 0) {
            nav('/orders/')
            return
        }
        setState({ page })
    })

    return (
        <div class='orders-fnd'>
            <div class='orders-tabs'>
                <button
                    classList={{ active: state.tab == 'star' }}
                    onClick={() => setState({ tab: 'star' })}
                >
                    استار
                </button>
                <button
                    classList={{ active: state.tab == 'phone' }}
                    onClick={() => setState({ tab: 'phone' })}
                >
                    شماره مجازی
                </button>
            </div>
            <Show when={state.count == 0}>
                <div class='order-empty'>
                    <h2>
                        <Show when={state.loading} fallback={'سفارشی یافت نشد'}>
                            درحال بارگزاری
                        </Show>
                    </h2>
                    <span>صفحه: {state.page}</span>
                </div>
            </Show>
            <Show
                when={state.tab == 'star'}
                fallback={
                    <Phones
                        page={state.page}
                        update={(loading, count) =>
                            setState({ loading, count })
                        }
                    />
                }
            >
                <Stars
                    page={state.page}
                    update={(loading, count) => setState({ loading, count })}
                />
            </Show>
            <div class='pagination'>
                <button
                    disabled={state.page < 1}
                    class='styled'
                    onClick={() => nav('/orders/' + (state.page - 1))}
                >
                    <ChevronLeftIcon />
                </button>
                <button
                    disabled={state.count < 32}
                    class='styled'
                    onClick={() => nav('/orders/' + (state.page + 1))}
                >
                    <ChevronRightIcon />
                </button>
            </div>
        </div>
    )
}

type Props = {
    page: number
    update(loading: boolean, count: number): void
}

const Stars: Component<Props> = P => {
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
                        <span class='value'>
                            {o.amount}
                            استار
                        </span>
                    </div>
                    <div class='row'>
                        <span class='key'>قیمت:</span>
                        <span class='value'>
                            <TomanDpy irr={o.cost} />
                            تومان
                        </span>
                    </div>
                </div>
            ))}
        </div>
    )
}

import { CountryDpy, ServiceDpy } from 'shared'

const Phones: Component<Props> = P => {
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
