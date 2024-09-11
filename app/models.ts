export type UserModel = {
    id: number
    name: string
    username: string | null
    auth_date: number
    wallet: number
    in_hold: number
    token: string
    photo: boolean
    admin: boolean
    banned: boolean
}

export type Transaction = {
    id: number
    user: number
    status: 'in_progress' | 'failed' | 'success'
    kind: 'in' | 'out'
    amount: number
    timestamp: number
    vendor_order_id: string | null
    vendor_track_id: number | null
    bank_track_id: number | null
    card: string | null
    card_hash: string | null
    date: number | null
}

export type Message = {
    id: number
    user: number
    activation_id: number
    text: string
    code: string
    country: string
    service: string
    seen: boolean
    received_at: string
    timestamp: number
}

export type OrderStatus = 'wating' | 'refunded' | 'done'

export type PhoneOrder = {
    activation_id: number
    cost: number
    country: string
    datetime: string
    id: number
    operator: string
    phone: string
    service: string
    status: OrderStatus
    user: number
}

export type StarOrder = {
    id: number
    user: number
    status: OrderStatus
    amount: number
    cost: number
    timestamp: number
}
