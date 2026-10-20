<template>
  <div class="max-w-3xl">
    <h2 class="mb-1 text-lg font-semibold">{{ t("about.title") }}</h2>
    <p class="mb-6 text-xs opacity-60">{{ t("about.subtitle") }}</p>

    <!-- 版本信息 / 检查更新：最常用的信息放在最前面 -->
    <section class="mb-6">
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

    <a-divider class="!my-5" />

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

    <!-- 联系方式：邮箱可唤起邮件客户端并支持一键复制，微信以二维码图片展示（可点击放大） -->
    <section>
      <div class="mb-3 text-sm font-medium">{{ t("about.contactTitle") }}</div>
      <a-card class="glass-card" size="small" :body-style="{ padding: '14px 16px' }">
        <div class="flex items-center gap-3">
          <MailOutlined class="text-2xl" />
          <div class="min-w-0 flex-1">
            <div class="text-xs opacity-60">{{ t("about.email") }}</div>
            <a
              class="contact-value"
              :href="`mailto:${CONTACT_EMAIL}`"
              @click.prevent="onOpenEmail"
            >
              {{ CONTACT_EMAIL }}
            </a>
          </div>
          <a-button size="small" :icon="h(CopyOutlined)" @click="onCopy(CONTACT_EMAIL)">
            {{ t("common.copy") }}
          </a-button>
        </div>

        <a-divider class="!my-3" />

        <div class="flex items-center gap-3">
          <WechatOutlined class="text-2xl" />
          <div class="min-w-0 flex-1">
            <div class="text-xs opacity-60">{{ t("about.wechat") }}</div>
            <div class="truncate text-sm font-medium">{{ t("about.wechatTip") }}</div>
          </div>
          <!-- 微信二维码：点击可用 a-image 内置的预览放大，方便用手机扫码 -->
          <a-image :src="wechatQr" :width="100" class="wechat-qr" />
        </div>
      </a-card>
      <p class="mt-2 text-xs opacity-60">{{ t("about.contactDesc") }}</p>
    </section>
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { message } from "ant-design-vue"
import {
  CopyOutlined,
  GithubOutlined,
  LinkOutlined,
  MailOutlined,
  WechatOutlined,
} from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import wechatQr from "@/assets/images/wx.webp"
import { getAppVersion, openExternal, openProjectRepo, PROJECT_GITHUB_URL } from "@/api/updater"
import { useUpdateStore } from "@/stores/update"

const { t } = useI18n()
const updateStore = useUpdateStore()

const version = ref("")

const stack = ["Tauri 2", "Vue 3", "TypeScript", "Rust", "Vite"]

/** 作者邮箱：展示、复制、唤起邮件客户端共用同一份常量 */
const CONTACT_EMAIL = "wu434425608@163.com"

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

/** 复制到剪贴板，失败时提示手动复制 */
async function onCopy(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    message.success(t("common.copySuccess"))
  } catch {
    message.error(t("common.copyFailed"))
  }
}

/** 用系统默认邮件客户端写信；没有可用客户端时退化为复制邮箱 */
async function onOpenEmail() {
  try {
    await openExternal(`mailto:${CONTACT_EMAIL}`)
  } catch {
    await onCopy(CONTACT_EMAIL)
  }
}
</script>

<style scoped>
/* 邮箱可点击：悬停时高亮，提示可以唤起邮件客户端 */
.contact-value {
  display: block;
  overflow: hidden;
  font-size: 13px;
  font-weight: 500;
  line-height: 1.5;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.2s ease;
}

.contact-value:hover {
  color: #6366f1;
  text-decoration: underline;
}

/* 微信二维码：白底圆角，深色模式下也保持足够对比度，便于手机扫描 */
.wechat-qr {
  flex-shrink: 0;
  padding: 4px;
  cursor: zoom-in;
  background-color: #fff;
  border: 1px solid rgba(15, 23, 42, 0.08);
  border-radius: 8px;
}

.wechat-qr :deep(img) {
  display: block;
  border-radius: 4px;
}

.dark .wechat-qr {
  border-color: rgba(255, 255, 255, 0.16);
}
</style>
