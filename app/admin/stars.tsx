import { TelegramStarIcon } from 'icons'
import { OrderStatus, StarOrder } from 'models'
import { STATUS_TABLE, TomanDpy, fmt_timestamp, httpx } from 'shared'
import { Show, onMount } from 'solid-js'
import { createStore, produce } from 'solid-js/store'

export default () => {
    type State = {
        orders: StarOrder[]
        loading: boolean
        page: number
        status: OrderStatus
        users: { [id: string]: string }
    }
    const [state, setState] = createStore<State>({
        orders: [],
        loading: false,
        page: 0,
        status: 'wating',
        users: {},
    })

    onMount(load)

    function load() {
        setState({ loading: true })
        httpx({
            url: '/api/admin/stars/',
            method: 'GET',
            params: {
                page: state.page,
                status: state.status,
            },
            onLoad(x) {
                setState({ loading: false })
                if (x.status != 200) return
                setState({ orders: x.response })
            },
        })
    }

    function get_username(id: number) {
        httpx({
            url: '/api/admin/users/username/',
            method: 'GET',
            params: { id },
            onLoad(x) {
                setState(
                    produce(s => {
                        s.users[id.toString()] = x.response
                    })
                )
            },
        })
    }

    return (
        <div class='admin admin-stars'>
            <div class='order-list'>
                {state.orders.map(o => (
                    <div class='order'>
                        <span>ایدی:</span>
                        <span>{o.id}</span>
                        <span>وضعیت:</span>
                        <span>{STATUS_TABLE[o.status]}</span>
                        <span>قیمت:</span>
                        <span>
                            <TomanDpy irr={o.cost} /> تومان
                        </span>
                        <span>تعداد:</span>
                        <span class='amount'>
                            {o.amount}
                            <TelegramStarIcon />
                        </span>
                        <span>تاریخ:</span>
                        <span class='n'>{fmt_timestamp(o.timestamp)}</span>
                        <span>کاربر:</span>
                        <Show
                            when={state.users[o.user]}
                            fallback={
                                <button
                                    class='styled'
                                    onClick={() => get_username(o.user)}
                                >
                                    {o.user}
                                </button>
                            }
                        >
                            <span>@{state.users[o.user]}</span>
                        </Show>
                    </div>
                ))}
            </div>
        </div>
    )
}
