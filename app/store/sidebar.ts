import { createEffect, createRoot } from 'solid-js'
import { JSX } from 'solid-js/jsx-runtime'

import { createStore } from 'solid-js/store'

type SidebarItem = {
    label: string
    icon: () => JSX.Element
    url: string
}

type Sidebar = {
    items: SidebarItem[]
    top: SidebarItem[]
    middle: SidebarItem
    bottom: SidebarItem[]
    active: number
    open: boolean
    show: boolean
}

const [sidebar, setSidebar] = createStore<Sidebar>({
    items: [],
    active: -1,
    open: localStorage.getItem('sidebar_open') != 'false',
    show: false,
    get top() {
        if (this.active == -1) return this.items
        return this.items.slice(0, this.active)
    },
    get middle() {
        return this.items[this.active]
    },
    get bottom() {
        if (this.active == -1) return []
        return this.items.slice(this.active + 1)
    },
})

createRoot(() => {
    createEffect(() => {
        localStorage.setItem('sidebar_open', '' + sidebar.open)
    })
})

export { sidebar, setSidebar }
