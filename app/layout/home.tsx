import './style/home.scss'
import { COUNTRY_LIST, Country } from './country-list'
import { SERVICE_LIST, Service } from './service-list'
import { Select } from 'comps'
import { createStore } from 'solid-js/store'
import { prices } from 'store'
import { Show, createMemo } from 'solid-js'

// const TIME_LIST: [number, string][] = [
//     20, 4, 12, 24, 48, 72, 96, 120, 144, 168, 192, 216, 240, 264, 288, 312, 336,
//     360, 384, 408, 432, 456, 480, 504, 528, 552, 576, 600, 624, 648, 672, 696,
//     720,
// ].map((t, i) => {
//     if (i === 0) return [t, t + ' دقیقه']
//
//     if (t >= 24) {
//         return [t, ~~(t / 24) + ' روز']
//     }
//
//     return [t, t + ' ساعت']
// })

export default () => {
    type State = {
        country: number | null
        service: string | null
    }

    const [state, setState] = createStore<State>({
        country: null,
        service: null,
    })

    function filter_country(country: Country) {
        if (!prices.update) return false

        let key = country[0].toString()
        if (state.service) {
            key += '-' + state.service
        }

        return key in prices.data
    }

    function filter_service(service: Service) {
        if (!prices.update) return false

        let key = service[0]
        if (state.country != null) {
            key = state.country + '-' + key
        }

        return key in prices.data
    }

    const selected = createMemo(() => {
        if (state.country == null || state.service == null) return null
        let key = state.country + '-' + state.service
        let value = prices.data[key]
        if (!value) return null
        return value[0]
    })

    return (
        <div class='home-fnd'>
            <div class='service'>
                <Select
                    disabled={!prices.update}
                    items={SERVICE_LIST.filter(filter_service).map(s => [
                        s[0],
                        <span
                            class='service-dpy'
                            style={
                                s[2]
                                    ? { direction: 'rtl' }
                                    : {
                                          direction: 'ltr',
                                          'font-family': 'var(--en)',
                                      }
                            }
                        >
                            {s[2] || s[1]}
                        </span>,
                    ])}
                    onChange={v => setState({ service: v[0] })}
                    placeholder='سرویس'
                    defaults={state.service ? [state.service] : undefined}
                />
            </div>
            <div class='country'>
                <Select
                    disabled={!prices.update}
                    items={COUNTRY_LIST.filter(filter_country).map(c => [
                        c[0],
                        <div class='country-dpy'>
                            <span class='name'>{c[4] + ' ' + c[3]}</span>
                            <span class='cc'>+{c[1]}</span>{' '}
                        </div>,
                    ])}
                    onChange={v => setState({ country: v[0] })}
                    placeholder='کشور'
                    defaults={
                        state.country != null ? [state.country] : undefined
                    }
                />
            </div>

            <div
                style={{
                    display: 'flex',
                    'flex-direction': 'column',
                    'font-family': 'var(--en)',
                }}
            >
                <span>country: {state.country}</span>
                <span>service: {state.service}</span>
                <span>
                    {prices.update} | {selected()}
                </span>
                <Show when={selected()}>
                    <button>
                        {(~~(selected() / 10)).toLocaleString()} تومان
                    </button>
                </Show>
            </div>
        </div>
    )
}
