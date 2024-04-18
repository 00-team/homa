import { defineConfig } from 'vite'
import type { WatcherOptions } from 'rollup'
import solidPlugin from 'vite-plugin-solid'

export default defineConfig(env => {
    let watch: WatcherOptions | null = null
    if (env.mode == 'development') {
        watch = {
            clearScreen: true,
        }
    }

    return {
        plugins: [solidPlugin({ hot: false })],
        server: {
            https: false,
            port: 8200,
            proxy: {
                '/api/': {
                    target: 'http://127.0.0.1:7200',
                    changeOrigin: true,
                },
            },
        },
        build: {
            target: 'esnext',
            outDir: 'dist',
            watch,
            assetsInlineLimit: 0,
            copyPublicDir: false,
        },
    }
})
