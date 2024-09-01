import { Component } from 'solid-js'
import { RouteSectionProps } from '@solidjs/router'
import Navbar from './nav'
import './style/dash.scss'

const Dash: Component<RouteSectionProps> = P => {
    return (
        <div class='dash-fnd'>
            <Navbar />
            <div class='sec-bottom'>{P.children}</div>
        </div>
    )
}

export default Dash
