import './style/index.scss'
import { Component } from 'solid-js'
import { A, RouteSectionProps } from '@solidjs/router'

const Admin: Component<RouteSectionProps> = P => {
    return (
        <div class='admin-fnd'>
            <div class='admin-tabs'>
                <A
                    href='/admin/'
                    classList={{ active: P.location.pathname == '/admin/' }}
                >
                    تنظیمات
                </A>
                <A href='/admin/stars/'>استار</A>
            </div>
            {P.children}
        </div>
    )
}

export default Admin
