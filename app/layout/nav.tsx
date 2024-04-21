import { A, useLocation } from '@solidjs/router'
import { Component, Show } from 'solid-js'

import './style/nav.scss'
import { self } from 'store'
import { UserIcon } from 'icons'

const Navbar: Component = () => {
    const location = useLocation()

    return (
        <nav class='navbar-fnd'>
            <div class='links'>
                <A href='/' classList={{ active: location.pathname == '/' }}>
                    Overview
                </A>
                <A href='/db/'>Database</A>
                <A href='/map/'>Map</A>
                <A href='/search/'>Search</A>
                <A href='/config/'>Config</A>
            </div>
            <A href='/profile/' class='session' draggable={false}>
                <div class='avatar'>
                    <Show when={self.user.photo} fallback={<UserIcon />}>
                        <img
                            draggable={false}
                            src={`/record/${self.user.id}.jpg`}
                        />
                    </Show>
                </div>
            </A>
        </nav>
    )
}

export default Navbar
