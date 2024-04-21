import { TelegramIcon } from 'icons'
import './style/login.scss'

export default () => {
    return (
        <div class='login-fnd'>
            <div class='login-form'>
                <h1>شماره مجازی تورا</h1>
                <span>ابتدا وارد شوید</span>
                <button
                    onclick={() => open('https://t.me/Thorabot?start=login')}
                >
                    ورود با تلگرام
                    <TelegramIcon />
                </button>
            </div>
        </div>
    )
}
