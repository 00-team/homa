import { COUNTRY_LIST } from 'layout/country-list'
import { SERVICE_LIST } from 'layout/service-list'
import { Component, Show, createMemo } from 'solid-js'

export const CountryDpy: Component<{ d: string }> = P => {
    const country = createMemo(() => {
        return COUNTRY_LIST.find(c => c[0].toString() == P.d)
    })
    return (
        <Show when={country()}>
            <span>
                {country()[3]} - {country()[4]}
            </span>
        </Show>
    )
}

export const ServiceDpy: Component<{ d: string }> = P => {
    const service = createMemo(() => {
        return SERVICE_LIST.find(s => s[0] == P.d)
    })
    return (
        <Show when={service()}>
            <span>{service()[2] || service()[1]}</span>
        </Show>
    )
}
