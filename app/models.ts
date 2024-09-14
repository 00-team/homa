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
    hash: string | null
}

export type GeneralModel = {
    money_gain: number
    money_loss: number
    money_total: number
    phone_tax: number
    price_diff_count: number
    price_diff_total: number
    prices: { [key: string]: [number, number, number, number] }
    prices_update: number
    rub_irr: number
    rub_irr_update: number
    star_tax: number
    usd_irr: number
    usd_irr_update: number
    disable_wallet: boolean
    disable_stars: boolean
    disable_phone: boolean
}

export const GENERAL_DEFAULT: GeneralModel = {
    money_gain: 0,
    money_loss: 0,
    money_total: 0,
    phone_tax: 0,
    price_diff_count: 0,
    price_diff_total: 0,
    prices: {},
    prices_update: 0,
    rub_irr: 0,
    rub_irr_update: 0,
    star_tax: 0,
    usd_irr: 0,
    usd_irr_update: 0,
    disable_wallet: false,
    disable_stars: false,
    disable_phone: false,
}
