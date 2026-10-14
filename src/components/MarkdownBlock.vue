<template>
  <!-- eslint-disable-next-line vue/no-v-html -->
  <div class="markdown-body" v-html="html" @click="onActivate" @auxclick="onActivate" />
</template>

<script setup lang="ts">
import { openExternal } from "@/api/updater"
import { renderMarkdown } from "@/utils/markdown"

const props = defineProps<{
  /** Markdown 源文本 */
  source?: string | null
}>()

const html = computed(() => renderMarkdown(props.source))

/** 拦截链接点击，统一用系统默认浏览器打开，避免 webview 自身跳转 */
function onActivate(e: MouseEvent) {
  const target = e.target
  const anchor = target instanceof Element ? target.closest("a") : null
  if (!(anchor instanceof HTMLAnchorElement)) return

  const href = anchor.getAttribute("href")
  if (!href || href.startsWith("#")) return

  e.preventDefault()
  void openExternal(href)
}
</script>
