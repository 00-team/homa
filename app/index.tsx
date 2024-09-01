import { Show, render } from 'solid-js/web'
import { self } from 'store'
import 'style/index.scss'
import Login from 'layout/login'
import Dash from 'layout/dash'
import Shop from 'layout/shop'
import Alert from 'comps/alert'
import { Route, Router } from '@solidjs/router'
import { lazy } from 'solid-js'

const Root = () => (
    <>
        <Show when={self.loged_in} fallback={<Login />}>
            <Router>
                <Route path='/' component={Dash}>
                    <Route path='/' component={Shop} />
                    <Route
                        path='/shop/stars/'
                        component={lazy(() => import('shop/stars'))}
                    />
                    <Route
                        path='/shop/virtual-number/'
                        component={lazy(() => import('shop/vnum'))}
                    />
                    <Route
                        path='/profile/'
                        component={lazy(() => import('layout/profile'))}
                    />
                    <Route
                        path='/messages/'
                        component={lazy(() => import('layout/messages'))}
                    />
                    <Route
                        path='/messages/:page'
                        component={lazy(() => import('layout/messages'))}
                    />
                    <Route
                        path='/orders/'
                        component={lazy(() => import('layout/orders'))}
                    />
                    <Route
                        path='/orders/:page'
                        component={lazy(() => import('layout/orders'))}
                    />
                    <Route
                        path='/transactions/'
                        component={lazy(() => import('layout/transactions'))}
                    />
                    <Route
                        path='/transactions/:page'
                        component={lazy(() => import('layout/transactions'))}
                    />
                    <Show when={self.user.admin}>
                        <Route
                            path='/admin'
                            component={lazy(() => import('admin'))}
                        />
                    </Show>
                </Route>
                <Route
                    path='*path'
                    component={lazy(() => import('layout/404'))}
                />
            </Router>
        </Show>
        <Alert />
    </>
)

render(Root, document.getElementById('root'))
