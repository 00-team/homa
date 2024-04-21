import { Show, render } from 'solid-js/web'

import { self } from 'store'
import 'style/index.scss'
import Login from 'layout/login'
import Alert from 'comps/alert'
import { Route, Router } from '@solidjs/router'
import NotFound from 'layout/404'

const Root = () => (
    <>
        <Show when={false && self.loged_in} fallback={<Login />}>
            <Router>
                <Route path='/' component={() => <h2>hi</h2>} />
                <Route path='*path' component={NotFound} />
            </Router>
        </Show>
        <Alert />
    </>
)

render(Root, document.getElementById('root'))
