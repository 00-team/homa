import { Component, onMount } from 'solid-js'

import './style/messages.scss'

import { createStore } from 'solid-js/store'
import { httpx } from 'shared'
import { Message } from 'models'

const Messages: Component = () => {
    type State = {
        messages: Message[]
    }

    const [state, setState] = createStore<State>({ messages: [] })

    onMount(() => {
        httpx({
            url: '/api/user/messages/',
            method: 'GET',
            type: 'json',
            onLoad(x) {
                console.log(x.response)
                setState({ messages: x.response })
            },
        })
    })

    return <div class='messages-fnd'>message</div>
}

export default Messages
