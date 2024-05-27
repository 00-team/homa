import { Show, createEffect } from 'solid-js'
import './style/profile.scss'
import { self } from 'store'
import { ChevronDownIcon, ChevronUpIcon, CirclePlusIcon, UserIcon } from 'icons'
import { createStore } from 'solid-js/store'
import { useSearchParams } from '@solidjs/router'

export default () => {
    type State = {
        add_amount: number
    }
    const [state, setState] = createStore<State>({ add_amount: 50 })
    const [SP, setSP] = useSearchParams()

    createEffect(() => {
        let addx = SP.add
        setSP({ add: null })

        if (!addx) return
        let add = parseInt(addx)
        if (isNaN(add) || add < 0 || add > 5000) return
        setState({ add_amount: add })
    })

    function add_amount(value: number) {
        setState(s => {
            let a = s.add_amount + value
            if (a < 0) a = 0
            if (a > 5000) a = 5000
            return { add_amount: a }
        })
    }

    function cash(value: number): string {
        return (~~(value / 10)).toLocaleString()
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
                    <div class='username'>
                        <span>نام کاربری: </span>
                        <span class='handle'>@{self.user.username}</span>
                    </div>
                </Show>
                <span class='wallet'>
                    کیف پول: {cash(self.user.wallet)} تومان
                </span>
                <span class='in-hold'>
                    در انتظار: {cash(self.user.in_hold)} تومان
                </span>
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
                <button class='icon'>
                    <CirclePlusIcon />
                </button>
            </div>
        </div>
    )
}
