import './style/home.scss'
import { COUNTRY_LIST } from './country-list'
import { SERVICE_LIST } from './service-list'
import { Select } from 'comps'
import { createStore } from 'solid-js/store'

export default () => {
    type State = {
        country: number | null
        service: string | null
    }

    const [state, setState] = createStore<State>({
        country: null,
        service: null,
    })

    return (
        <div class='home-fnd'>
            <div class='country'>
                <Select
                    items={COUNTRY_LIST.map(c => (
                        <div class='country-dpy'>
                            <span class='name'>{c[4] + ' ' + c[3]}</span>
                            <span class='cc'>+{c[1]}</span>
                        </div>
                    ))}
                    onChange={v => setState({ country: COUNTRY_LIST[v[0]][0] })}
                    placeholder='کشور'
                />
            </div>
            <div class='service'>
                <Select
                    items={SERVICE_LIST.map(s => (
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
                        </span>
                    ))}
                    onChange={v => setState({ service: SERVICE_LIST[v[0]][0] })}
                    placeholder='سرویس'
                />
            </div>
            <div style={{ display: 'flex', 'flex-direction': 'column' }}>
                <span>country: {state.country}</span>
                <span>service: {state.service}</span>
            </div>
        </div>
    )
}
