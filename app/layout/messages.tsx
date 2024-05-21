import { Component, Show, createEffect, onMount } from 'solid-js'

import './style/messages.scss'

import { createStore } from 'solid-js/store'
import { httpx } from 'shared'
import { Message } from 'models'
import { useNavigate, useParams } from '@solidjs/router'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'

const Messages: Component = () => {
    type State = {
        messages: Message[]
        page: number
    }

    const [state, setState] = createStore<State>({ messages: [], page: 0 })
    const UP = useParams()
    const nav = useNavigate()

    createEffect(() => {
        let page = parseInt(UP.page || '0')
        if (isNaN(page) || page < 0) {
            nav('/messages/')
            return
        }
        fetch_messages(page)
    })

    function fetch_messages(page: number) {
        httpx({
            url: '/api/user/messages/?page=' + page,
            method: 'GET',
            type: 'json',
            onLoad(x) {
                if (x.status != 200) return

                setState({ messages: x.response, page })
            },
        })
    }

    return (
        <div class='messages-fnd'>
            <Show when={state.messages.length == 0}>
                <div class='message-empty'>
                    <h2>پیامی یافت نشد</h2>
                    <span>صفحه: {state.page}</span>
                </div>
            </Show>
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
            <div class='pagination'>
                <Show when={state.page > 0}>
                    <button
                        class='styled'
                        onClick={() => nav('/messages/' + (state.page - 1))}
                    >
                        <ChevronLeftIcon />
                    </button>
                </Show>
                <Show when={state.messages.length >= 32}>
                    <button
                        class='styled'
                        onClick={() => nav('/messages/' + (state.page + 1))}
                    >
                        <ChevronRightIcon />
                    </button>
                </Show>
            </div>
        </div>
    )
}

export default Messages
