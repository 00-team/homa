import { createStore } from 'solid-js/store'
import './style/index.scss'
import { httpx } from 'shared'
import { Component, onMount } from 'solid-js'

export default () => {
    type State = {
        usd_irr: number
        rub_irr: number
        star_tax: number
        phone_tax: number
    }
    const [state, setState] = createStore<State>({
        usd_irr: 0,
        rub_irr: 0,
        star_tax: 0,
        phone_tax: 0,
    })

    onMount(() => {
        load()
    })

    function load() {
        httpx({
            url: '/api/admin/general/',
            method: 'GET',
            onLoad(x) {
                if (x.status != 200) return
                setState({ ...x.response })
            },
        })
    }

    function save() {
        httpx({
            url: '/api/admin/general/',
            method: 'PATCH',
            json: {
                rub_irr: state.rub_irr,
                usd_irr: state.usd_irr,
                star_tax: state.star_tax,
                phone_tax: state.phone_tax,
            },
            onLoad(x) {
                if (x.status != 200) return
                load()
            },
        })
    }

    return (
        <div class='admin-fnd'>
            <NumberInput
                label='usd to irr'
                value={state.usd_irr}
                onUpdate={v => setState({ usd_irr: v })}
            />
            <NumberInput
                label='rub to irr'
                value={state.rub_irr}
                onUpdate={v => setState({ rub_irr: v })}
            />
            <NumberInput
                label='star tax'
                value={state.star_tax}
                onUpdate={v => setState({ star_tax: v })}
            />
            <NumberInput
                label='phone tax'
                value={state.phone_tax}
                onUpdate={v => setState({ phone_tax: v })}
            />
            <button class='save-btn' onClick={save}>
                Save
            </button>
        </div>
    )
}

type NumberInputProps = {
    value: number
    onUpdate(value: number): void
    label: string
}
const NumberInput: Component<NumberInputProps> = P => {
    return (
        <div class='input'>
            <span>{P.label}</span>
            <input
                type='number'
                value={P.value}
                onChange={v => P.onUpdate(parseInt(v.currentTarget.value) || 0)}
            />
        </div>
    )
}
