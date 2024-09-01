import './style/vnum.scss'
import { COUNTRY_LIST, Country } from 'shared/country-list'
import { SERVICE_LIST, Service } from 'shared/service-list'
import { Select } from 'comps'
import { createStore } from 'solid-js/store'
import { prices, self } from 'store'
import { Match, Show, Switch, createEffect, createMemo } from 'solid-js'
import { RotateCcwIcon } from 'icons'
import { useNavigate } from '@solidjs/router'
import { TomanDpy, httpx } from 'shared'

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
    const nav = useNavigate()

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

    createEffect(() => {
        console.log(selected())
    })

    const service = createMemo(() => {
        if (state.service == null) return null
        let s = SERVICE_LIST.find(s => s[0] == state.service)
        if (!s) return null
        return s[2] || s[1]
    })

    const country = createMemo(() => {
        if (state.country == null) return null
        let c = COUNTRY_LIST.find(c => c[0] == state.country)
        if (!c) return null
        return c[3]
    })

    function avg(key: string): string {
        let cost = prices.data[key][0]
        let count = prices.data[key][1]

        let a = ~~(cost / count / 1e4) * 1e3

        return a.toLocaleString()
    }

    function buy() {
        if (selected() > self.user.wallet) {
            location.replace(
                '/api/user/deposit/?amount=' + (selected() - self.user.wallet)
            )
            // nav('/profile/?add=' + (selected() - self.user.wallet + 7e4))
            return
        }

        httpx({
            url: '/api/vendor/buy/',
            method: 'POST',
            params: {
                country: state.country,
                service: state.service,
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
        <div class='shop-vnum-fnd'>
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
                    placeholder='انتخاب سرویس'
                    selected={state.service ? [state.service] : []}
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
                    placeholder='انتخاب کشور'
                    selected={state.country != null ? [state.country] : []}
                />
            </div>
            <div class='prices'>
                <Show when={service()}>
                    <div class='row'>
                        <span>میانگین قیمت {service()}:</span>
                        <span class='n'>{avg(state.service)} تومان</span>
                    </div>
                </Show>
                <Show when={country()}>
                    <div class='row'>
                        <span>میانگین قیمت {country()}:</span>
                        <span class='n'>{avg(state.country + '')} تومان</span>
                    </div>
                </Show>
            </div>
            <div class='actions'>
                <button class='buy' disabled={!selected()} onClick={buy}>
                    <Switch>
                        <Match when={selected() == null}>
                            سرویس و کشور را انتخاب کنید
                        </Match>
                        <Match when={selected() > self.user.wallet}>
                            شارژ کیف پول{' '}
                            <TomanDpy irr={selected() - self.user.wallet} />{' '}
                            تومان
                        </Match>
                        <Match when={selected() <= self.user.wallet}>
                            خرید <TomanDpy irr={selected()} /> تومان
                        </Match>
                    </Switch>
                </button>
                <button
                    disabled={state.service == null && state.country == null}
                    class='reset'
                    onclick={() => setState({ service: null, country: null })}
                >
                    <RotateCcwIcon />
                </button>
            </div>
        </div>
    )
}
