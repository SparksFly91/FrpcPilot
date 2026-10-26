import { defineStore } from "pinia"
import { preferences } from "@/utils/preferences"

export const useOnboardingStore = defineStore("onboarding", () => {
  const dismissed = useStorage("frpcpilot:onboarding-dismissed", false, preferences)
  const ready = ref(false)
  const active = ref(false)

  function start() {
    active.value = true
  }

  function close() {
    active.value = false
    dismissed.value = true
  }

  return { dismissed, ready, active, start, close }
})
