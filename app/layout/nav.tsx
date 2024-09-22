import { A } from '@solidjs/router'
import { Show, onMount } from 'solid-js'

import './style/nav.scss'
import { self } from 'store'
import {
    CirclePlusIcon,
    HistoryIcon,
    MailWarningIcon,
    UserIcon,
    MailOpenIcon,
    SwordIcon,
    ShopIcon,
    ShoppingBasketIcon,
} from 'icons'
import { createStore } from 'solid-js/store'
import { TomanDpy, httpx } from 'shared'

export const Navbar = () => {
    type State = {
        messages: number
    }

    const [state, setState] = createStore<State>({ messages: 0 })

    onMount(() => {
        httpx({
            url: '/api/user/messages-unseen-count/',
            method: 'GET',
            onLoad(x) {
                setState({ messages: x.response })
            },
        })
    })

    return (
        <nav class='nav-fnd navbar-fnd'>
            <div class='left'>
                <A href='/'>
                    <ShopIcon />
                </A>
                <A href='/profile/' class='avatar'>
                    <Show when={self.user.photo} fallback={<UserIcon />}>
                        <img
                            draggable={false}
                            src={`/record/u-${self.user.id}.jpg`}
                        />
                    </Show>
                </A>
                <A href='/transactions/'>
                    <HistoryIcon />
                </A>
                <A href='/messages/'>
                    <Show when={state.messages} fallback={<MailOpenIcon />}>
                        <span class='mail-icon'>
                            <span class='mail-icon-count'>
                                <Show when={state.messages < 10} fallback={'+'}>
                                    {state.messages}
                                </Show>
                            </span>
                            <MailWarningIcon />
                        </span>
                    </Show>
                </A>
                <A href='/orders/'>
                    <ShoppingBasketIcon />
                </A>
                <Show when={self.user.admin}>
                    <A href='/admin/'>
                        <SwordIcon />
                    </A>
                </Show>
            </div>
            <div class='right'>
                <A href='/profile/' class='money'>
                    <span>
                        <TomanDpy irr={self.user.wallet} />
                    </span>
                    <CirclePlusIcon />
                </A>
            </div>
        </nav>
    )
}

export const NavTop = () => {
    return (
        <nav class='nav-fnd navbar-fnd'>
            <div class='left '>
                <h2 class='nav-title'>تورا</h2>
            </div>

            <div class='right'>
                <A href='/profile/' class='money'>
                    <span>
                        <TomanDpy irr={self.user.wallet} />
                    </span>
                    <CirclePlusIcon />
                </A>
            </div>
        </nav>
    )
}

export const NavBottom = () => {
    type State = {
        messages: number
    }

    const [state, setState] = createStore<State>({ messages: 0 })

    onMount(() => {
        httpx({
            url: '/api/user/messages-unseen-count/',
            method: 'GET',
            onLoad(x) {
                setState({ messages: x.response })
            },
        })
    })

    return (
        <nav class='nav-fnd nav-bottom-fnd'>
            <A href='/profile/' class='avatar'>
                <Show when={self.user.photo} fallback={<UserIcon />}>
                    <img
                        draggable={false}
                        src={`/record/u-${self.user.id}.jpg`}
                    />
                </Show>
            </A>
            <A href='/transactions/'>
                <HistoryIcon />
            </A>
            <A href='/'>
                <ShopIcon />
            </A>
            <A href='/messages/'>
                <Show when={state.messages} fallback={<MailOpenIcon />}>
                    <span class='mail-icon'>
                        <span class='mail-icon-count'>
                            <Show when={state.messages < 10} fallback={'+'}>
                                {state.messages}
                            </Show>
                        </span>
                        <MailWarningIcon />
                    </span>
                </Show>
            </A>
            <A href='/orders/'>
                <ShoppingBasketIcon />
            </A>
            <Show when={self.user.admin}>
                <A href='/admin/'>
                    <SwordIcon />
                </A>
            </Show>
        </nav>
    )
}
