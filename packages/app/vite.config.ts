import tailwindcss from '@tailwindcss/vite';
import devtools from 'solid-devtools/vite';
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
    plugins: [wasm(), solid(), devtools(), tailwindcss()],
});
