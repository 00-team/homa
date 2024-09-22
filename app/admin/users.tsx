import { useSearchParams } from '@solidjs/router'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'
import { UserModel } from 'models'
import { TomanDpy, fmt_timestamp, httpx } from 'shared'
import { Component, Show, createEffect, onMount } from 'solid-js'
import { createStore } from 'solid-js/store'

import './style/users.scss'

export default () => {
    type State = {
        users: UserModel[]
        loading: boolean
        page: number
    }
    const [state, setState] = createStore<State>({
        users: [],
        loading: false,
        page: 0,
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
            url: '/api/admin/users/',
            method: 'GET',
            params: {
                page,
            },
            onLoad(x) {
                setState({ loading: false })
                if (x.status != 200) return
                setState({ users: x.response })
                document.querySelector('.user-list').scrollTo(0, 0)
            },
        })
    }

    return (
        <div class='admin admin-users'>
            {/*<div class='order-status'>
                {Object.entries(STATUS_TABLE).map(([os, label]) => (
                    <button
                        onClick={() => setState({ status: os as OrderStatus })}
                        classList={{ active: state.status == os }}
                    >
                        {label}
                    </button>
                ))}
            </div>*/}
            <div class='user-list'>
                {state.users.map(u => (
                    <User u={u} />
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
                    disabled={state.users.length < 32}
                    class='styled'
                    onClick={() => setState(s => ({ page: s.page + 1 }))}
                >
                    <ChevronRightIcon />
                </button>
            </div>
        </div>
    )
}

type UserProps = {
    u: UserModel
}
const User: Component<UserProps> = P => {
    return (
        <div class='user'>
            <div class='image'>
                <img
                    draggable={false}
                    loading='lazy'
                    decoding='async'
                    src={
                        P.u.photo
                            ? `/record/u-${P.u.id}.jpg`
                            : '/static/default-user.jpg'
                    }
                />
            </div>
            <div class='info'>
                <span>ایدی:</span>
                <span>{P.u.id}</span>

                <span>نام:</span>
                <span> {P.u.name} </span>

                <span>کیف پول:</span>
                <span>
                    <TomanDpy irr={P.u.wallet} /> تومان
                </span>

                <span>نام کاربری</span>
                <Show when={P.u.username} fallback={<span>❌</span>}>
                    <a
                        class='n username'
                        href={`https://t.me/${P.u.username}`}
                        target='_blank'
                    >
                        @{P.u.username}
                    </a>
                </Show>

                <span>تاریخ ورود:</span>
                <span class='n'>{fmt_timestamp(P.u.auth_date)}</span>
            </div>
        </div>
    )
}
