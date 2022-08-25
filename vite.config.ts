import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

// 引入自动导入式ui框架
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
//自动导入icons
import IconsResolver from 'unplugin-icons/resolver'
import Icons from 'unplugin-icons/vite'
import { resolve } from '@tauri-apps/api/path'

// import { vueResolve } from  "path"
// import styleImport, { VantResolve } from "vite-plugin-style-import"

export default defineConfig({
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
  plugins: [vue(),
    // ui自动导入
    AutoImport({
      resolvers: [
        // 自动导入 Element Plus 相关函数，如：ElMessage, ElMessageBox... (带样式)
        ElementPlusResolver(),
        // 自动导入图标组件
        IconsResolver({
          prefix: 'Icon',
        })
      ],
    }),
    Components({
      resolvers: [
        // 自动注册图标组件
        IconsResolver({
          enabledCollections: ['ep'],
        }),
        // 自动导入 Element Plus 组件
        ElementPlusResolver(),
      ],
    }),

    Icons({
      autoInstall: true,
    }),
    // 快捷路径插件
    // styleImport({
    //   resolves: [VantResolve()],
    // }),
  ],
  // 快捷路径
  // resolve: {
  //   alias: {
  //     "@": vueResolve("src"),
  //   },
  // },
})