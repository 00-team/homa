import { Component, Show, createSignal, onCleanup, onMount } from 'solid-js'
import { RouteSectionProps } from '@solidjs/router'
import { NavTop, NavBottom, Navbar } from './nav'
import './style/dash.scss'

const NAVMAX = 500
const Dash: Component<RouteSectionProps> = P => {
    const [width, setWidth] = createSignal(innerWidth)

    function update_width() {
        setWidth(innerWidth)
    }

    onMount(() => {
        window.addEventListener('resize', update_width)
    })
    onCleanup(() => {
        window.removeEventListener('resize', update_width)
    })

    return (
        <div class='dash-fnd' classList={{ mobile: width() < NAVMAX }}>
            <Show when={width() > NAVMAX} fallback={<NavTop />}>
                <Navbar />
            </Show>
            <div class='sec-bottom'>{P.children}</div>
            <Show when={width() < NAVMAX}>
                <NavBottom />
            </Show>
        </div>
    )
}

export default Dash
