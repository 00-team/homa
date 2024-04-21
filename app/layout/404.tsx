import { A, RouteSectionProps } from '@solidjs/router'
import { Component } from 'solid-js'

import './style/404.scss'

const NotFound: Component<RouteSectionProps> = P => {
    return (
        <div class='not-found-fnd'>
            <h1>Not Found</h1>
            <p>path: '/{P.params.path}' was not found</p>
            <A href='/'>Home</A>
        </div>
    )
}
export default NotFound
