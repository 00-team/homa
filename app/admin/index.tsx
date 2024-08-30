import { createStore } from 'solid-js/store'
import './style/index.scss'
import { httpx } from 'shared'
import { onMount } from 'solid-js'

export default () => {
    type State = {
        usd_irr: number
        rub_irr: number
    }
    const [state, setState] = createStore<State>({
        usd_irr: 0,
        rub_irr: 0,
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
            },
            onLoad(x) {
                if (x.status != 200) return
                load()
            },
        })
    }

    return (
        <div class='admin-fnd'>
            <div class='input'>
                <span>usd to irr</span>
                <input
                    type='number'
                    value={state.usd_irr}
                    onChange={v =>
                        setState({
                            usd_irr: parseInt(v.currentTarget.value) || 0,
                        })
                    }
                />
            </div>
            <div class='input'>
                <span>rub to irr</span>
                <input
                    type='number'
                    value={state.rub_irr}
                    onChange={v =>
                        setState({
                            rub_irr: parseInt(v.currentTarget.value) || 0,
                        })
                    }
                />
            </div>
            <button class='save-btn' onClick={save}>
                Save
            </button>
        </div>
    )
}
