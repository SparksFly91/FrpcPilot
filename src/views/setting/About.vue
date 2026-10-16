<template>
  <div class="max-w-3xl">
    <h2 class="mb-1 text-lg font-semibold">{{ t("about.title") }}</h2>
    <p class="mb-6 text-xs opacity-60">{{ t("about.subtitle") }}</p>

    <!-- 项目介绍 -->
    <section class="mb-6">
      <div class="mb-2 text-sm font-medium">{{ t("about.introTitle") }}</div>
      <p class="text-sm leading-relaxed opacity-80">{{ t("about.intro") }}</p>
      <div class="mt-3 flex flex-wrap gap-2">
        <a-tag v-for="tech in stack" :key="tech" color="blue" class="!me-0">{{ tech }}</a-tag>
      </div>
    </section>

    <a-divider class="!my-5" />

    <!-- 使用流程 -->
    <section class="mb-6">
      <div class="mb-3 text-sm font-medium">{{ t("about.flowTitle") }}</div>
      <a-steps direction="vertical" size="small" :current="flowItems.length" :items="flowItems" />
    </section>

    <a-divider class="!my-5" />

    <!-- 开源地址 -->
    <section class="mb-6">
      <div class="mb-3 text-sm font-medium">{{ t("about.repoTitle") }}</div>
      <a-card class="glass-card" size="small" :body-style="{ padding: '14px 16px' }">
        <div class="flex items-center gap-3">
          <GithubOutlined class="text-2xl" />
          <div class="min-w-0 flex-1">
            <div class="truncate text-sm font-medium">SparksFly91 / FrpcPilot</div>
            <div class="truncate text-xs opacity-60">{{ PROJECT_GITHUB_URL }}</div>
          </div>
          <a-button type="primary" size="small" :icon="h(LinkOutlined)" @click="onOpenRepo">
            {{ t("about.openRepo") }}
          </a-button>
        </div>
      </a-card>
      <p class="mt-2 text-xs opacity-60">{{ t("about.repoDesc") }}</p>
    </section>

    <a-divider class="!my-5" />

    <!-- 版本信息 / 检查更新 -->
    <section>
      <div class="mb-3 text-sm font-medium">{{ t("about.versionTitle") }}</div>
      <div class="flex flex-wrap items-center gap-3">
        <span class="text-xs opacity-60">{{ t("about.currentVersion") }}</span>
        <a-tag color="purple" class="!me-0">v{{ version }}</a-tag>
        <a-badge :dot="updateStore.hasUpdate">
          <a-button size="small" :loading="updateStore.checking" @click="onCheckUpdate">
            {{ t("about.checkUpdate") }}
          </a-button>
        </a-badge>
        <span v-if="updateStore.hasUpdate" class="text-xs text-[#6366f1]">
          {{ t("about.newVersion", { version: updateStore.info?.version }) }}
        </span>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { GithubOutlined, LinkOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import { getAppVersion, openProjectRepo, PROJECT_GITHUB_URL } from "@/api/updater"
import { useUpdateStore } from "@/stores/update"

const { t } = useI18n()
const updateStore = useUpdateStore()

const version = ref("")

const stack = ["Tauri 2", "Vue 3", "TypeScript", "Rust", "Vite"]

const flowItems = computed(() => [
  { title: t("about.flow.step1Title"), description: t("about.flow.step1Desc") },
  { title: t("about.flow.step2Title"), description: t("about.flow.step2Desc") },
  { title: t("about.flow.step3Title"), description: t("about.flow.step3Desc") },
  { title: t("about.flow.step4Title"), description: t("about.flow.step4Desc") },
])

onMounted(async () => {
  version.value = await getAppVersion()
})

function onOpenRepo() {
  void openProjectRepo()
}

function onCheckUpdate() {
  void updateStore.checkForUpdate(false)
}
</script>
