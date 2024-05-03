import { Show } from 'solid-js'
import './style/profile.scss'
import { self } from 'store'
import { ChevronDownIcon, ChevronUpIcon, UserIcon } from 'icons'
import { createStore } from 'solid-js/store'

export default () => {
    type State = {
        add_amount: number
    }
    const [state, setState] = createStore<State>({ add_amount: 0 })

    function add_amount(value: number) {
        setState(s => {
            let a = s.add_amount + value
            if (a < 0) a = 0
            if (a > 5000) a = 5000
            return { add_amount: a }
        })
    }

    return (
        <div class='profile-fnd'>
            <div class='img'>
                <Show when={self.user.photo} fallback={<UserIcon />}>
                    <img
                        draggable={false}
                        src={`/record/${self.user.id}.jpg`}
                    />
                </Show>
            </div>
            <div class='info'>
                <span class='name'>{self.user.name}</span>
                <Show when={self.user.username}>
                    <span class='username'>@{self.user.username}</span>
                </Show>
                <span class='wallet'>کیف پول: {self.user.wallet}</span>
                <span class='in-hold'>در انتظار: {self.user.in_hold}</span>
            </div>
            <div class='add-wallet'>
                <div class='up-down'>
                    <button class='icon' onclick={() => add_amount(+5)}>
                        <ChevronUpIcon />
                    </button>
                    <button class='icon' onclick={() => add_amount(-5)}>
                        <ChevronDownIcon />
                    </button>
                </div>
                <span class='amount'>{state.add_amount} هزار تومان</span>
            </div>
        </div>
    )
}
