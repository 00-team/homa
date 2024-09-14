import { GENERAL_DEFAULT, GeneralModel } from 'models'
import { TomanDpy, httpx } from 'shared'
import { Component, Show, createMemo, onMount } from 'solid-js'
import { createStore, produce } from 'solid-js/store'

export default () => {
    type State = {
        loading: boolean
        G: GeneralModel
    }
    const [state, setState] = createStore<State>({
        loading: true,
        G: GENERAL_DEFAULT,
    })

    onMount(load)

    function load() {
        setState({ loading: true })
        httpx({
            url: '/api/admin/general/',
            method: 'GET',
            onLoad(x) {
                setState({ loading: false })
                if (x.status != 200) return
                setState({ G: x.response })
            },
        })
    }

    function save() {
        if (state.loading) return

        httpx({
            url: '/api/admin/general/',
            method: 'PATCH',
            json: {
                rub_irr: state.G.rub_irr,
                usd_irr: state.G.usd_irr,
                star_tax: state.G.star_tax,
                phone_tax: state.G.phone_tax,
                disable_wallet: state.G.disable_wallet,
                disable_stars: state.G.disable_stars,
                disable_phone: state.G.disable_phone,
            },
            onLoad(x) {
                if (x.status != 200) return
                load()
            },
        })
    }

    function uG(value: Partial<GeneralModel>) {
        setState(
            produce(s => {
                s.G = { ...s.G, ...value }
            })
        )
    }

    const avg_diff = createMemo(() => {
        if (state.G.price_diff_count == 0 || state.G.price_diff_total == 0) {
            return 10
        }
        return state.G.price_diff_total / state.G.price_diff_count
    })

    return (
        <div class='admin admin-general' classList={{ loading: state.loading }}>
            <Show when={state.loading}>
                <span class='loading-msg'>درحال بارگزاری</span>
            </Show>

            <div class='inputs'>
                <span>سود: </span>
                <span>
                    <TomanDpy irr={state.G.money_gain} />
                </span>
                <span>ضرر: </span>
                <span>{state.G.money_loss}</span>
                <span>مجموع: </span>
                <span>{state.G.money_total}</span>
                <span>میانگین تغییر قیمت: </span>
                <span>{avg_diff()}</span>
                <NumberInput
                    min={1}
                    label='قیمت دلار به ریال'
                    value={state.G.usd_irr}
                    onUpdate={v => uG({ usd_irr: v })}
                />
                <NumberInput
                    min={1}
                    label='قیمت روبل به ریال'
                    value={state.G.rub_irr}
                    onUpdate={v => uG({ rub_irr: v })}
                />
                <NumberInput
                    min={0}
                    max={999}
                    label='کارمزد استار'
                    value={state.G.star_tax}
                    onUpdate={v => uG({ star_tax: v })}
                />
                <NumberInput
                    min={0}
                    max={999}
                    label='کارمزد شماره مجازی'
                    value={state.G.phone_tax}
                    onUpdate={v => uG({ phone_tax: v })}
                />
                <BoolInput
                    value={state.G.disable_wallet}
                    label='غیرفعال کردن کیف پول'
                    onUpdate={v => uG({ disable_wallet: v })}
                />
                <BoolInput
                    value={state.G.disable_stars}
                    label='غیرفعال کردن استار'
                    onUpdate={v => uG({ disable_stars: v })}
                />
                <BoolInput
                    value={state.G.disable_phone}
                    label='غیرفعال کردن شماره مجازی'
                    onUpdate={v => uG({ disable_phone: v })}
                />
            </div>
            <Show when={!state.loading}>
                <button class='styled save-btn' onClick={save}>
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
            <span>{P.label}</span>
            <input
                class='styled'
                type='text'
                value={P.value.toLocaleString()}
                onInput={e => {
                    let s = e.currentTarget.value.replaceAll(',', '')
                    let v = parseInt(s) || 0
                    if (P.min != undefined && v < P.min) v = P.min
                    if (P.max != undefined && v > P.max) v = P.max
                    e.currentTarget.value = v.toString()
                    P.onUpdate(v)
                }}
            />
        </>
    )
}

type BoolInputProps = {
    value: boolean
    onUpdate(value: boolean): void
    label: string
}
const BoolInput: Component<BoolInputProps> = P => {
    return (
        <>
            <span>{P.label}</span>
            <input
                class='styled'
                type='checkbox'
                checked={P.value}
                onChange={e => {
                    P.onUpdate(e.currentTarget.checked)
                }}
            />
        </>
    )
}
