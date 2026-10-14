<template>
  <div class="flex items-center gap-1">
    <!-- 语言切换 -->
    <a-dropdown placement="bottomRight" trigger="click">
      <a-button
        type="text"
        class="tb-btn"
        :title="t('titlebar.language')"
      >
        <template #icon><TranslationOutlined /></template>
      </a-button>
      <template #overlay>
        <a-menu
          :selected-keys="[appStore.language]"
          @click="onLanguageClick"
        >
          <a-menu-item
            v-for="item in languageOptions"
            :key="item.key"
          >
            {{ item.label }}
          </a-menu-item>
        </a-menu>
      </template>
    </a-dropdown>

    <!-- 明暗切换 -->
    <a-button
      type="text"
      class="tb-btn"
      :title="appStore.isDark ? t('titlebar.lightMode') : t('titlebar.darkMode')"
      @click="appStore.toggleDark()"
    >
      <template #icon>
        <BulbFilled v-if="appStore.isDark" />
        <BulbOutlined v-else />
      </template>
    </a-button>

    <!-- 检查更新 -->
    <a-button
      type="text"
      class="tb-btn"
      :title="t('titlebar.checkUpdate')"
      :loading="updateStore.checking"
      @click="onCheckUpdate"
    >
      <template #icon><CloudSyncOutlined /></template>
    </a-button>

    <!-- GitHub -->
    <a-button
      type="text"
      class="tb-btn"
      :title="t('titlebar.github')"
      @click="onOpenGithub"
    >
      <template #icon><GithubOutlined /></template>
    </a-button>

    <!-- 设置 -->
    <a-button
      type="text"
      class="tb-btn"
      :title="t('titlebar.settings')"
      @click="emit('settings')"
    >
      <template #icon><SettingOutlined /></template>
    </a-button>

    <!-- 最小化（macOS 由左侧红绿灯负责，此处隐藏） -->
    <a-button
      v-if="!isMac"
      type="text"
      class="tb-btn"
      :title="t('titlebar.minimize')"
      @click="minimizeWindow()"
    >
      <template #icon><MinusOutlined /></template>
    </a-button>

    <!-- 关闭（macOS 由左侧红绿灯负责，此处隐藏） -->
    <a-button
      v-if="!isMac"
      type="text"
      class="tb-btn tb-btn-close"
      :title="t('titlebar.close')"
      @click="closeWindow()"
    >
      <template #icon><CloseOutlined /></template>
    </a-button>
  </div>
</template>

<script setup lang="ts">
import {
  BulbFilled,
  BulbOutlined,
  CloseOutlined,
  CloudSyncOutlined,
  GithubOutlined,
  MinusOutlined,
  SettingOutlined,
  TranslationOutlined,
} from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { MenuProps } from "ant-design-vue"
import { useAppStore } from "@/stores/app"
import { useUpdateStore } from "@/stores/update"
import { languageOptions, type Language } from "@/i18n"
import { closeWindow, minimizeWindow } from "@/utils/window"
import { openProjectRepo } from "@/api/updater"
import { usePlatform } from "@/composables/usePlatform"

const emit = defineEmits<{ settings: [] }>()

const { t } = useI18n()
const appStore = useAppStore()
const updateStore = useUpdateStore()
const { isMac } = usePlatform()

const onLanguageClick: MenuProps["onClick"] = ({ key }) => {
  appStore.setLanguage(key as Language)
}

/** 顶部栏手动检查更新（非静默，无更新 / 失败都会给出提示） */
function onCheckUpdate() {
  void updateStore.checkForUpdate(false)
}

/** 打开项目 GitHub 仓库 */
function onOpenGithub() {
  void openProjectRepo()
}
</script>

<style scoped lang="scss">
.tb-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 28px;
  width: 28px;
  padding: 0;
  border-radius: 8px;
  color: rgba(15, 23, 42, 0.75);
  transition: background-color 0.2s ease, color 0.2s ease;

  &:hover {
    background-color: rgba(15, 23, 42, 0.08) !important;
    color: rgba(15, 23, 42, 0.95);
  }

  &-close:hover {
    background-color: #ef4444 !important;
    color: #fff !important;
  }
}

.dark .tb-btn {
  color: rgba(226, 232, 240, 0.8);

  &:hover {
    background-color: rgba(255, 255, 255, 0.12) !important;
    color: #fff;
  }
}
</style>
