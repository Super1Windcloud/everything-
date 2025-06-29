/// <reference types="vite/client" />
/**
 * /**告诉 TypeScript 编译器在编译时包含 vite/client 的类型定义。
 * vite/client 提供了一些特定于 Vite 的类型声明，例如一些全局变量的类型、
 * 模块热替换（HMR）的接口等。这对于在 TypeScript 项目中使用
 * Vite 构建工具是非常重要的。*/
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

/**
 * 声明 .vue 文件模块
 * 在 TypeScript 中，默认无法识别 .vue 文件（Vue 的单文件组件），
 * 因此需要通过模块声明告诉 TypeScript 如何处理这种文件类型。
 * 2. 定义 .vue 文件的导出类型
 * 这段代码声明：所有以 .vue 结尾的导入（
 * 如 import MyComponent from "./MyComponent.vue"）
 * 都应该被视为 Vue 的组件，类型是 DefineComponent。
 * 3. 提供类型支持
 * DefineComponent 是 Vue 3 提供的类型，它定义了组件的结构
 * （如 props、data、methods 等），使 TypeScript 能对
 * Vue 组件进行类型检查和智能提示。
 * 4. 兼容 Vue 3 的 Composition API 和 Options API
 * DefineComponent<{}, {}, any> 表示：
 * 第一个 {}：组件没有 props（可以替换为具体的 props 类型）。
 * 第二个 {}：组件没有 data（可以替换为 data 的类型）。
 * any：组件的其他选项（如 methods、computed 等）可以是任意类型。
 * */
