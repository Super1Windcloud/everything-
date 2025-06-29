import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

const host = process.env.TAURI_DEV_HOST;
const component = path.resolve(__dirname, "src");

export default defineConfig(async () => ({
    plugins: [vue({
        template :{
            compilerOptions :{
                compatConfig :{
                    MODE: 2
                }
            }
        }

    })],

    clearScreen: false,
    server: {
        port: 3333,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
    resolve: {
        alias: {
            vue: '@vue/compat',
            "@": component
            ,
            "~": path.resolve(__dirname, "src-tauri")

        },
        extensions: [".ts", ".tsx", ".js", ".jsx", ".json", ".vue"]
    }
}));
