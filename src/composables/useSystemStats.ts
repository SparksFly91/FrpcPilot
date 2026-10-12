import { invoke } from "@tauri-apps/api/core"
import { isTauri } from "@/utils/window"

interface SystemInfo {
  cpu: number
  memory: number
}

/**
 * 系统资源占用（CPU / 内存 百分比）。
 *
 * 优先调用后端命令 `get_system_info`（返回 { cpu, memory }），
 * 后端尚未实现时回退到模拟数据，保证界面可用。
 */
export function useSystemStats(interval = 2000) {
  const cpu = ref(0)
  const memory = ref(0)
  const isMock = ref(true)

  // 模拟数据：随机游走，避免界面显示为静态 0
  let mockCpu = 18
  let mockMem = 42
  function mock() {
    mockCpu = Math.min(95, Math.max(3, mockCpu + (Math.random() - 0.5) * 8))
    mockMem = Math.min(95, Math.max(10, mockMem + (Math.random() - 0.5) * 4))
    cpu.value = Math.round(mockCpu)
    memory.value = Math.round(mockMem)
    isMock.value = true
  }

  async function refresh() {
    if (isTauri()) {
      try {
        const res = await invoke<SystemInfo>("get_system_info")
        if (res) {
          cpu.value = Math.round(res.cpu)
          memory.value = Math.round(res.memory)
          isMock.value = false
          return
        }
      } catch {
        // 后端命令未实现，回退到模拟数据
      }
    }
    mock()
  }

  const { pause, resume } = useIntervalFn(refresh, interval, { immediate: true })

  return { cpu, memory, isMock, refresh, pause, resume }
}
