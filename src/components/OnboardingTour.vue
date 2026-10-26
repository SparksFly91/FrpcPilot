<template>
  <a-tour
    :open="onboarding.active && visible"
    :current="current"
    :steps="steps"
    :gap="{ offset: 8, radius: 10 }"
    :mask="{ color: 'rgba(15, 23, 42, 0.55)' }"
    @change="changeStep"
    @close="onboarding.close"
  />
</template>

<script setup lang="ts">
import type { TourProps } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { useOnboardingStore } from "@/stores/onboarding"

const { t } = useI18n()
const router = useRouter()
const onboarding = useOnboardingStore()
const current = ref(0)
const visible = ref(false)
const navigating = ref(false)
const target = shallowRef<HTMLElement>()
const destinations = [
  { path: "/servers", target: "server-create", key: "servers" },
  { path: "/tunnels", target: "tunnel-create", key: "tunnels" },
  { path: "/download", target: "frp-download", key: "download" },
  { path: "/dashboard", target: "dashboard-start", key: "dashboard" },
] as const

const steps = computed<TourProps["steps"]>(() => destinations.map((step, index) => ({
  title: t(`onboarding.${step.key}Title`),
  description: t(`onboarding.${step.key}Description`),
  target: index === current.value ? target.value : undefined,
  placement: "bottom",
})))

async function changeStep(index: number) {
  const destination = destinations[index]
  if (!destination || navigating.value) return
  navigating.value = true
  visible.value = false
  try {
    await router.push(destination.path)
    await nextTick()
    current.value = index
    target.value = document.querySelector<HTMLElement>(`[data-tour="${destination.target}"]`) ?? undefined
    visible.value = onboarding.active
  } catch (error) {
    onboarding.close()
    console.error("Unable to open onboarding page", error)
  } finally {
    navigating.value = false
  }
}

watch(() => onboarding.active, (active) => {
  if (active) void changeStep(0)
  else visible.value = false
})

watch(() => onboarding.ready, (ready) => {
  if (ready && !onboarding.dismissed) onboarding.start()
}, { immediate: true })

// Antdv 提供关闭按钮；同时允许键盘用户通过 Escape 跳过引导。
onKeyStroke("Escape", () => {
  if (onboarding.active) onboarding.close()
})
</script>
