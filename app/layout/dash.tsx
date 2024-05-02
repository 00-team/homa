import { Component } from 'solid-js'
import { RouteSectionProps } from '@solidjs/router'
import Navbar from './nav'
import './style/dash.scss'

const Dash: Component<RouteSectionProps> = P => {
    return (
        <div class='dash-fnd'>
            <Navbar />
            <div class='sec-bottom'>
                <div class='sec-content'>
                    <div class='sec-wrapper'>{P.children}</div>
                </div>
            </div>
        </div>
    )
}

export default Dash
