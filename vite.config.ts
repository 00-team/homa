import { defineConfig } from 'vite'
import type { WatcherOptions } from 'rollup'
import solidPlugin from 'vite-plugin-solid'

import tsconfigPaths from 'vite-tsconfig-paths'

export default defineConfig(env => {
    let watch: WatcherOptions | null = null
    if (env.mode == 'development') {
        watch = {
            clearScreen: true,
        }
    }

    return {
        plugins: [tsconfigPaths(), solidPlugin({ hot: false })],
        root: 'app',
        server: {
            https: false,
            port: 8000,
            proxy: {
                '/api/': {
                    target: 'http://127.0.0.1:7000',
                    changeOrigin: true,
                },
                '/record/': {
                    target: 'http://127.0.0.1:7000',
                    changeOrigin: true,
                },
            },
        },
        build: {
            target: 'esnext',
            outDir: '../web/dist',
            watch,
            assetsInlineLimit: 0,
            copyPublicDir: false,
            emptyOutDir: true,
        },
    }
})
