import { useSearchParams } from '@solidjs/router'
import { Confact } from 'comps/confact'
import {
    BanIcon,
    ChevronLeftIcon,
    ChevronRightIcon,
    CircleCheckIcon,
    TelegramStarIcon,
} from 'icons'
import { OrderStatus, StarOrder } from 'models'
import { STATUS_TABLE, TomanDpy, fmt_timestamp, httpx } from 'shared'
import { Component, Match, Switch, createEffect, onMount } from 'solid-js'
import { createStore, produce } from 'solid-js/store'

type Usernames = { [id: string]: string | -1 }

export default () => {
    type State = {
        orders: StarOrder[]
        loading: boolean
        page: number
        status: OrderStatus
        usernames: Usernames
    }
    const [state, setState] = createStore<State>({
        orders: [],
        loading: false,
        page: 0,
        status: 'wating',
        usernames: {},
    })
    const [sp, setSp] = useSearchParams()

    onMount(() => {
        let page = parseInt(sp.page || '0') || 0
        if (page != state.page) {
            setState({ page: page })
        } else {
            load(page)
        }
    })
    createEffect(() => {
        setSp({ page: state.page })
        load(state.page)
    })

    function load(page: number) {
        setState({ loading: true })
        httpx({
            url: '/api/admin/stars/',
            method: 'GET',
            params: {
                page,
                status: state.status,
            },
            onLoad(x) {
                setState({ loading: false })
                if (x.status != 200) return
                setState({ orders: x.response })
                document.querySelector('.order-list').scrollTo(0, 0)
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
                        if (x.response != 200) {
                            s.usernames[id.toString()] = -1
                        } else {
                            s.usernames[id.toString()] = x.response || -1
                        }
                    })
                )
            },
        })
    }

    return (
        <div class='admin admin-stars'>
            <div class='order-list'>
                {state.orders.map((o, idx) => (
                    <Order
                        o={o}
                        username={state.usernames[o.user]}
                        get_username={() => get_username(o.user)}
                        update={no =>
                            setState(
                                produce(s => {
                                    s.orders[idx] = no
                                })
                            )
                        }
                    />
                ))}
            </div>
            <div class='pagination'>
                <button
                    disabled={state.page < 1}
                    class='styled'
                    onClick={() => setState(s => ({ page: s.page - 1 }))}
                >
                    <ChevronLeftIcon />
                </button>
                <button
                    disabled={state.orders.length < 32}
                    class='styled'
                    onClick={() => setState(s => ({ page: s.page + 1 }))}
                >
                    <ChevronRightIcon />
                </button>
            </div>
        </div>
    )
}

type OrderProps = {
    o: StarOrder
    username: string | -1
    update(order: StarOrder): void
    get_username(): void
}
const Order: Component<OrderProps> = P => {
    type State = {
        hash: string
    }
    const [state, setState] = createStore<State>({
        hash: '',
    })

    type UOA = { hash: string; status: 'done' } | { status: 'refunded' }
    function update_order(args: UOA) {
        httpx({
            url: `/api/admin/stars/${P.o.id}/`,
            method: 'PATCH',
            json: args,
            onLoad(x) {
                if (x.response != 200) return
                P.update(x.response)
            },
        })
    }

    return (
        <div class='order'>
            <div class='info'>
                <span>ایدی:</span>
                <span>{P.o.id}</span>
                <span>وضعیت:</span>
                <span>{STATUS_TABLE[P.o.status]}</span>
                <span>قیمت:</span>
                <span>
                    <TomanDpy irr={P.o.cost} /> تومان
                </span>
                <span>تعداد:</span>
                <span class='amount'>
                    {P.o.amount}
                    <TelegramStarIcon />
                </span>
                <span>تاریخ:</span>
                <span class='n'>{fmt_timestamp(P.o.timestamp)}</span>
                <span>کاربر:</span>
                <Switch>
                    <Match when={!P.username}>
                        <button class='styled n' onClick={P.get_username}>
                            {P.o.user}
                        </button>
                    </Match>
                    <Match when={P.username == -1}>
                        <span>❌</span>
                    </Match>
                    <Match when={P.username}>
                        <span class='n'>@{P.username}</span>
                    </Match>
                </Switch>
                <span>هش:</span>
                <textarea
                    class='styled n'
                    rows={2}
                    value={state.hash}
                    onInput={e => {
                        setState({ hash: e.currentTarget.value })
                    }}
                />
            </div>
            <div class='actions'>
                <Confact
                    icon={BanIcon}
                    color='var(--red)'
                    timer_ms={2000}
                    onAct={() => update_order({ status: 'refunded' })}
                />
                <Confact
                    icon={CircleCheckIcon}
                    disabled={!state.hash}
                    timer_ms={2000}
                    color='var(--green)'
                    onAct={() =>
                        update_order({ status: 'done', hash: state.hash })
                    }
                />
            </div>
        </div>
    )
}
