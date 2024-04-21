import { A } from '@solidjs/router'
import { Component, Show } from 'solid-js'

import './style/nav.scss'
import { self } from 'store'
import {
    CirclePlusIcon,
    HistoryIcon,
    MailWarningIcon,
    PhoneIcon,
    UserIcon,
    HomeIcon,
} from 'icons'

const Navbar: Component = () => {
    return (
        <nav class='nav-fnd'>
            <div class='left'>
                <A href='/'>
                    <HomeIcon />
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
                    <MailWarningIcon />
                </A>
                <A href='/numbers/'>
                    <PhoneIcon />
                </A>
            </div>
            <div class='right'>
                <A href='/profile/' class='money'>
                    <span>{self.user.wallet.toLocaleString()}</span>
                    <CirclePlusIcon />
                </A>
            </div>
        </nav>
    )
}

export default Navbar
