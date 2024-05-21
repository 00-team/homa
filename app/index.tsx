import { Show, render } from 'solid-js/web'

import { self } from 'store'
import 'style/index.scss'
import Login from 'layout/login'
import Dash from 'layout/dash'
import Home from 'layout/home'
import Profile from 'layout/profile'
import Alert from 'comps/alert'
import { Route, Router } from '@solidjs/router'
import NotFound from 'layout/404'
import Messages from 'layout/messages'

const Root = () => (
    <>
        <Show when={self.loged_in} fallback={<Login />}>
            <Router>
                <Route path='/' component={Dash}>
                    <Route path='/' component={Home} />
                    <Route path='/profile/' component={Profile} />
                    <Route path='/messages/' component={Messages} />
                    <Route path='/messages/:page' component={Messages} />
                </Route>
                <Route path='*path' component={NotFound} />
            </Router>
        </Show>
        <Alert />
    </>
)

render(Root, document.getElementById('root'))
