import { createStore } from 'solid-js/store'
import './style/stars.scss'
import { TomanDpy, httpx } from 'shared'
import { onMount } from 'solid-js'
import { TelegramStarIcon } from 'icons'
import { self } from 'store'
import { useNavigate } from '@solidjs/router'
import { addAlert } from 'comps/alert'

export default () => {
    type State = {
        price: number
    }
    const [state, setState] = createStore<State>({
        price: 0,
    })
    const nav = useNavigate()

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

    function buy(amount: number) {
        if (!self.user.username) {
            addAlert({
                type: 'error',
                timeout: 7,
                content: '',
                subject: 'بدون یوزرنیم خرید استار امکان پذیر نمی باشد',
            })
            return
        }
        let price = amount * state.price
        if (price > self.user.wallet) {
            location.replace(
                '/api/user/deposit/?amount=' + (price - self.user.wallet)
            )
            // nav('/profile/?add=' + (selected() - self.user.wallet + 7e4))
            return
        }

        httpx({
            url: '/api/stars/buy/',
            method: 'POST',
            json: {
                amount,
            },
            onLoad(x) {
                if (x.status == 200) {
                    nav('/orders/')
                    return
                }
            },
        })
    }

    return (
        <div class='shop-stars-fnd'>
            {[50, 75, 100, 150, 250, 550, 2500, 4000].map(s => (
                <button class='styled row-btn' onclick={() => buy(s)}>
                    <span>
                        <TelegramStarIcon /> {s} استار
                    </span>
                    <span>
                        <TomanDpy irr={s * state.price} /> تومان
                    </span>
                </button>
            ))}
        </div>
    )
}
