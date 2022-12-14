import Vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import path from 'path'
// 引入自动导入式ui框架
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
//自动导入icons
import IconsResolver from 'unplugin-icons/resolver'
import Icons from 'unplugin-icons/vite'
import { resolve } from '@tauri-apps/api/path'
import Inspect from 'vite-plugin-inspect'
// import { vueResolve } from  "path"
// import styleImport, { VantResolve } from "vite-plugin-style-import"
const pathSrc = path.resolve(__dirname, 'src')
export default defineConfig({
  // ref:: https://github.com/sxzz/element-plus-best-practices/blob/db2dfc983ccda5570033a0ac608a1bd9d9a7f658/vite.config.ts#L21-L58
  // 防止 vite 输出复杂的 rust 错误
  clearScreen: false,
  // Tauri 使用固定端口，若此端口不可用将会导致程序错误
  server: {
    strictPort: true,
  },
  // 使用 `TAURI_PLATFORM`、`TAURI_ARCH`、`TAURI_FAMILY`,
  // `TAURI_PLATFORM_VERSION`、`TAURI_PLATFORM_TYPE` 和 `TAURI_DEBUG` 环境变量
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri 支持 es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // 不为调试构建压缩构建体积
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // 为调试构建生成源代码映射 (sourcemap)
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  base: "/",
  resolve: {
    alias: {
      '@': pathSrc,
    },
  },
  plugins: [
    Vue(),
    // ui自动导入
    AutoImport({
      // Auto import functions from Vue, e.g. ref, reactive, toRef...
      // 自动导入 Vue 相关函数，如：ref, reactive, toRef 等
      imports: ['vue'],
      // Auto import functions from Element Plus, e.g. ElMessage, ElMessageBox... (with style)
      // 自动导入 Element Plus 相关函数，如：ElMessage, ElMessageBox... (带样式)
      resolvers: [
        // 自动导入 Element Plus 相关函数，如：ElMessage, ElMessageBox... (带样式)
        ElementPlusResolver(),
        // Auto import icon components
        // 自动导入图标组件
        IconsResolver({
          prefix: 'icon',
        })
      ],
      dts: path.resolve(pathSrc, 'auto-imports.d.ts'),
    }),
    Components({
      resolvers: [
        // Auto register icon components
        // 自动注册图标组件
        IconsResolver({
          enabledCollections: ['ep'],
        }),
        // Auto register Element Plus components
        // 自动导入 Element Plus 组件
        ElementPlusResolver(),
      ],
      dts: path.resolve(pathSrc, 'components.d.ts'),
    }),

    Icons({
      compiler: 'vue3',
      autoInstall: true,
    }),
    // 快捷路径插件
    // styleImport({
    //   resolves: [VantResolve()],
    // }),
    Inspect(),
  ],
  // 快捷路径
  // resolve: {
  //   alias: {
  //     "@": vueResolve("src"),
  //   },
  // },
})