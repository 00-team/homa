import { A } from '@solidjs/router'
import './style/shop.scss'

export default () => {
    return (
        <div class='shop-fnd'>
            <div class='shop-card'>
                <img src='/static/telegram-stars.png' />
                <h2>تلگرام استار</h2>
                <A href='/shop/stars/'>خرید</A>
            </div>
            <div class='shop-card'>
                <img src='/static/telegram-stars.png' />
                <h2>شماره مجازی</h2>
                <A href='/shop/vnum/'>خرید</A>
            </div>
            <div class='shop-card'>
                <img src='/static/telegram-stars.png' />
                <h2>تلگرام پریمیوم</h2>
                <A href='/shop/premium/'>خرید</A>
            </div>
        </div>
    )
}
