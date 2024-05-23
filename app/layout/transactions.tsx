import { Transaction } from 'models'
import './style/transactions.scss'
import { useNavigate, useParams } from '@solidjs/router'
import { createStore } from 'solid-js/store'
import { createEffect } from 'solid-js'
import { httpx } from 'shared'

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

    return (
        <div class='transactions-fnd'>
            <div class='transaction-list'>
                {state.transactions.map(t => (
                    <div class='transaction'>{t.amount}</div>
                ))}
            </div>
        </div>
    )
}
