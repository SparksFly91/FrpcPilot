import { initializePreferences } from "@/utils/preferences"

// 状态仓库创建时会读取初始设置，因此先加载安装目录内保存的设置。
async function start() {
  try {
    await initializePreferences()
    await import("./bootstrap")
  } catch (error) {
    console.error("Failed to initialize application", error)
    const loader = document.getElementById("startup-loader")
    if (loader) loader.textContent = `无法读取应用设置 / Cannot load settings: ${String(error)}`
  }
}

void start()
