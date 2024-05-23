import { Transaction } from 'models'
import './style/transactions.scss'
import { useNavigate, useParams } from '@solidjs/router'
import { createStore } from 'solid-js/store'
import { createEffect } from 'solid-js'
import { fmt_timestamp, httpx } from 'shared'

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
                    <div class='transaction'>
                        <span>
                            مبلغ: {(~~(t.amount / 10)).toLocaleString()} تومان
                        </span>
                        <span>تاریخ: {fmt_timestamp(t.timestamp)}</span>
                        <span>نوع: {KIND_TABLE[t.kind]}</span>
                        <span>وضعیت: {STATUS_TABLE[t.status]}</span>
                        <span>پیگیری: {t.vendor_track_id || '---'}</span>
                        <span>پیگیری بانک: {t.bank_track_id || '---'}</span>
                    </div>
                ))}
            </div>
        </div>
    )
}
