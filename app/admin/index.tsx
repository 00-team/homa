import { createStore } from 'solid-js/store'
import './style/index.scss'
import { httpx } from 'shared'
import { Component, Show, onMount } from 'solid-js'

export default () => {
    type State = {
        usd_irr: number
        rub_irr: number
        star_tax: number
        phone_tax: number
        loading: boolean
    }
    const [state, setState] = createStore<State>({
        loading: true,
        usd_irr: 1,
        rub_irr: 1,
        star_tax: 0,
        phone_tax: 0,
    })

    onMount(() => {
        load()
    })

    function load() {
        setState({ loading: true })
        httpx({
            url: '/api/admin/general/',
            method: 'GET',
            onLoad(x) {
                if (x.status != 200) return
                setState({ ...x.response, loading: false })
            },
        })
    }

    function save() {
        if (state.loading) return

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
        <div class='admin-fnd' classList={{ loading: state.loading }}>
            <Show when={state.loading}>
                <span class='loading-msg'>درحال بارگزاری</span>
            </Show>
            <div class='inputs'>
                <NumberInput
                    min={1}
                    label='قیمت دلار به ریال'
                    value={state.usd_irr}
                    onUpdate={v => setState({ usd_irr: v })}
                />
                <NumberInput
                    min={1}
                    label='قیمت روبل به ریال'
                    value={state.rub_irr}
                    onUpdate={v => setState({ rub_irr: v })}
                />
                <NumberInput
                    min={0}
                    max={999}
                    label='کارمزد استار'
                    value={state.star_tax}
                    onUpdate={v => setState({ star_tax: v })}
                />
                <NumberInput
                    min={0}
                    max={999}
                    label='کارمزد شماره مجازی'
                    value={state.phone_tax}
                    onUpdate={v => setState({ phone_tax: v })}
                />
            </div>
            <Show when={!state.loading}>
                <button class='save-btn' onClick={save}>
                    ذخیره
                </button>
            </Show>
        </div>
    )
}

type NumberInputProps = {
    value: number
    onUpdate(value: number): void
    label: string
    min?: number
    max?: number
}
const NumberInput: Component<NumberInputProps> = P => {
    return (
        <>
            <input
                type='number'
                value={P.value}
                min={P.min}
                max={P.max}
                onChange={e => {
                    let v = parseInt(e.currentTarget.value) || 0
                    if (P.min != undefined && v < P.min) v = P.min
                    if (P.max != undefined && v > P.max) v = P.max
                    e.currentTarget.value = v.toString()
                    P.onUpdate(v)
                }}
            />
            <span>{P.label}</span>
        </>
    )
}
