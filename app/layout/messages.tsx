import { Component, createEffect, onMount } from 'solid-js'

import './style/messages.scss'

import { createStore } from 'solid-js/store'
import { httpx } from 'shared'
import { Message } from 'models'
import { useParams } from '@solidjs/router'

const Messages: Component = () => {
    type State = {
        messages: Message[]
        page: number
    }

    const [state, setState] = createStore<State>({ messages: [], page: 0 })
    const UP = useParams()

    createEffect(() => {
        let page = parseInt(UP.page || '0') || 0
        fetch_messages(page)
    })

    function fetch_messages(page: number) {
        httpx({
            url: '/api/user/messages/?page=' + page,
            method: 'GET',
            type: 'json',
            onLoad(x) {
                setState({ messages: x.response, page })
            },
        })
    }

    return (
        <div class='messages-fnd'>
            <div class='message-list'>
                {state.messages.map(m => (
                    <div class='message'>
                        text: {m.text}
                        <br />
                        code: {m.code}
                        <br />
                        id: {m.id}
                        <br />
                        aid: {m.activation_id}
                        <br />
                        timestamp: {m.timestamp}
                        <br />
                        rec: {m.received_at}
                        <br />
                        country: {m.country}
                        <br />
                        service: {m.service}
                        <br />
                    </div>
                ))}
            </div>
        </div>
    )
}

export default Messages
