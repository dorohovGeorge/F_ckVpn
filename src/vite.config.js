import { defineConfig } from 'vite'
import dns from 'dns'

dns.setDefaultResultOrder('verbatim')

export default defineConfig({
    server: {
        port: 8080,
        host: 'localhost'
    }
});