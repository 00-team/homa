import { Show, createEffect } from 'solid-js'
import { createStore } from 'solid-js/store'
import { useNavigate, useParams } from '@solidjs/router'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'
import { Stars } from './stars'
import { Phones } from './phone'

import './style/index.scss'

export default () => {
    type State = {
        tab: 'star' | 'phone'
        page: number
        count: number
        loading: boolean
    }
    const [state, setState] = createStore<State>({
        tab: 'star',
        page: 0,
        count: 0,
        loading: true,
    })
    const UP = useParams()
    const nav = useNavigate()

    createEffect(() => {
        let page = parseInt(UP.page || '0')
        if (isNaN(page) || page < 0) {
            nav('/orders/')
            return
        }
        setState({ page })
    })

    return (
        <div class='orders-fnd'>
            <div class='orders-tabs'>
                <button
                    classList={{ active: state.tab == 'star' }}
                    onClick={() => setState({ tab: 'star' })}
                >
                    استار
                </button>
                <button
                    classList={{ active: state.tab == 'phone' }}
                    onClick={() => setState({ tab: 'phone' })}
                >
                    شماره مجازی
                </button>
            </div>
            <Show when={state.count == 0}>
                <div class='order-empty'>
                    <h2>
                        <Show when={state.loading} fallback={'سفارشی یافت نشد'}>
                            درحال بارگزاری
                        </Show>
                    </h2>
                    <span>صفحه: {state.page}</span>
                </div>
            </Show>
            <Show
                when={state.tab == 'star'}
                fallback={
                    <Phones
                        page={state.page}
                        update={(loading, count) =>
                            setState({ loading, count })
                        }
                    />
                }
            >
                <Stars
                    page={state.page}
                    update={(loading, count) => setState({ loading, count })}
                />
            </Show>
            <div class='pagination'>
                <button
                    disabled={state.page < 1}
                    class='styled'
                    onClick={() => nav('/orders/' + (state.page - 1))}
                >
                    <ChevronLeftIcon />
                </button>
                <button
                    disabled={state.count < 32}
                    class='styled'
                    onClick={() => nav('/orders/' + (state.page + 1))}
                >
                    <ChevronRightIcon />
                </button>
            </div>
        </div>
    )
}
