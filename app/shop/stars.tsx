import { createStore } from 'solid-js/store'
import './style/stars.scss'
import { TomanDpy, httpx } from 'shared'
import { onMount } from 'solid-js'

export default () => {
    type State = {
        price: number
    }
    const [state, setState] = createStore<State>({
        price: 0,
    })

    onMount(load)

    function load() {
        httpx({
            url: '/api/stars/price/',
            method: 'GET',
            onLoad(x) {
                if (x.status != 200) return
                setState({ price: x.response })
            },
        })
    }

    return (
        <div class='shop-stars-fnd'>
            {[50, 75, 100, 150, 250, 350, 500, 750, 1000, 1500, 2500].map(s => (
                <div>
                    {s} | <TomanDpy irr={s * state.price} /> تومان
                </div>
            ))}
        </div>
    )
}
