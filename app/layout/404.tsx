import { A } from '@solidjs/router'

import './style/404.scss'

const NotFound = () => {
    return (
        <div class='not-found-fnd'>
            <h1>پیدا نشد</h1>
            <A href='/'>خانه</A>
        </div>
    )
}
export default NotFound
