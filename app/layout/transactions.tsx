import { Transaction } from 'models'
import './style/transactions.scss'
import { useNavigate, useParams } from '@solidjs/router'
import { createStore } from 'solid-js/store'
import { Show, createEffect } from 'solid-js'
import { fmt_timestamp, httpx } from 'shared'
import { ChevronLeftIcon, ChevronRightIcon } from 'icons'

export default () => {
    type State = {
        transactions: Transaction[]
        page: number
    }

    const [state, setState] = createStore<State>({ transactions: [], page: 0 })
    const UP = useParams()
    const nav = useNavigate()

    createEffect(() => {
        let page = parseInt(UP.page || '0')
        if (isNaN(page) || page < 0) {
            nav('/transactions/')
            return
        }
        fetch_transactions(page)
    })

    function fetch_transactions(page: number) {
        httpx({
            url: '/api/user/transactions/?page=' + page,
            method: 'GET',
            type: 'json',
            onLoad(x) {
                if (x.status != 200) return

                setState({ transactions: x.response, page })
            },
        })
    }

    const KIND_TABLE: { [k in Transaction['kind']]: string } = {
        in: 'ورودی',
        out: 'خروجی',
    }
    const STATUS_TABLE: { [k in Transaction['status']]: string } = {
        failed: 'ناموفق',
        success: 'موفق',
        in_progress: 'در حال انجام',
    }

    return (
        <div class='transactions-fnd'>
            <div class='transaction-list'>
                {state.transactions.map(t => (
                    <div class='transaction' classList={{ [t.status]: true }}>
                        <div class='row'>
                            <span class='key'>مبلغ:</span>
                            <span class='value'>
                                {(~~(t.amount / 10)).toLocaleString()} تومان
                            </span>
                        </div>
                        <div class='row'>
                            <span class='key'>تاریخ:</span>
                            <span class='value datetime'>
                                {fmt_timestamp(t.timestamp)}
                            </span>
                        </div>

                        <div class='row'>
                            <span class='key'>نوع:</span>
                            <span class='value'>{KIND_TABLE[t.kind]}</span>
                        </div>
                        <div class='row'>
                            <span class='key'>وضعیت:</span>
                            <span class='value'>{STATUS_TABLE[t.status]}</span>
                        </div>
                        <div class='row'>
                            <span class='key'>پیگیری:</span>
                            <span class='value'>
                                {t.vendor_track_id || '---'}
                            </span>
                        </div>
                        <div class='row'>
                            <span class='key'>پیگیری بانک:</span>
                            <span class='value'>
                                {t.bank_track_id || '---'}
                            </span>
                        </div>
                    </div>
                ))}
            </div>
            <div class='pagination'>
                <Show when={state.page > 0}>
                    <button
                        class='styled'
                        onClick={() => nav('/transactions/' + (state.page - 1))}
                    >
                        <ChevronLeftIcon />
                    </button>
                </Show>
                <Show when={state.transactions.length >= 32}>
                    <button
                        class='styled'
                        onClick={() => nav('/transactions/' + (state.page + 1))}
                    >
                        <ChevronRightIcon />
                    </button>
                </Show>
            </div>
        </div>
    )
}
