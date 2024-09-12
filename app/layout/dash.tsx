import { Component, Show, createEffect, createSignal } from 'solid-js'
import { onCleanup, onMount } from 'solid-js'
import { RouteSectionProps, useSearchParams } from '@solidjs/router'
import { NavTop, NavBottom, Navbar } from './nav'
import './style/dash.scss'
import { addAlert } from 'comps/alert'

const NAVMAX = 500
const Dash: Component<RouteSectionProps> = P => {
    const [width, setWidth] = createSignal(innerWidth)
    const [sp, setSp] = useSearchParams()

    createEffect(() => {
        let err = sp.error
        if (err) {
            addAlert({ type: 'error', subject: err, timeout: 700, content: '' })
            setSp({ error: null })
        }
    })

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
