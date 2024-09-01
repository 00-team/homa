import { A } from '@solidjs/router'
import { Component, Show, onMount } from 'solid-js'

import './style/nav.scss'
import { self } from 'store'
import {
    CirclePlusIcon,
    HistoryIcon,
    MailWarningIcon,
    PhoneIcon,
    UserIcon,
    HomeIcon,
    MailOpenIcon,
    SwordIcon,
    ShopIcon,
} from 'icons'
import { createStore } from 'solid-js/store'
import { httpx } from 'shared'

const Navbar: Component = () => {
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
        <nav class='nav-fnd'>
            <div class='left'>
                <A href='/'>
                    <ShopIcon />
                </A>
                <A href='/profile/' class='avatar'>
                    <Show when={self.user.photo} fallback={<UserIcon />}>
                        <img
                            draggable={false}
                            src={`/record/${self.user.id}.jpg`}
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
                    <PhoneIcon />
                </A>
                <Show when={self.user.admin}>
                    <A href='/admin/'>
                        <SwordIcon />
                    </A>
                </Show>
            </div>
            <div class='right'>
                <A href='/profile/' class='money'>
                    <span>{(~~(self.user.wallet / 10)).toLocaleString()}</span>
                    <CirclePlusIcon />
                </A>
            </div>
        </nav>
    )
}

export default Navbar
