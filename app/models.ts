export type UserModel = {
    id: number
    name: string
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
    kind: number // in OR out | withdrawl OR deposit
    status: number // success | failed | in progress
    amount: number
    vendor_order_id: string | null
    vendor_track_id: number | null
    card_number: string | null
    hashed_card_number: string | null
    date: number | null
    bank_track_id: number | null
}
