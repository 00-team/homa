import './style/home.scss'
import { COUNTRY_LIST } from './country-list'
import { SERVICE_LIST } from './service-list'
import { Select } from 'comps'

export default () => {
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
                    onChange={v => console.log(v)}
                    placeholder='کشور'
                />
            </div>
            <div class='service'>
                <Select
                    items={SERVICE_LIST.map(s => s[1])}
                    onChange={v => console.log(SERVICE_LIST[v[0]])}
                    placeholder='سرویس'
                />
            </div>
        </div>
    )
}
