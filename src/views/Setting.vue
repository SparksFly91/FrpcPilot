<template>
  <div class="glass-panel flex h-full min-h-0">
    <!-- 左侧：设置菜单 -->
    <aside class="w-44 shrink-0 border-r border-slate-200/70 p-2 dark:border-white/10">
      <a-menu
        class="setting-menu"
        mode="inline"
        :selected-keys="selectedKeys"
        :items="menuItems"
        @click="onMenuClick"
      />
    </aside>

    <!-- 右侧：设置内容 -->
    <section class="min-w-0 flex-1 overflow-y-auto p-6">
      <router-view />
    </section>
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { BgColorsOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { MenuProps } from "ant-design-vue"

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const selectedKeys = computed(() => [route.path])

const menuItems = computed<MenuProps["items"]>(() => [
  {
    key: "/setting/appearance",
    icon: () => h(BgColorsOutlined),
    label: t("settings.menu.appearance"),
  },
])

const onMenuClick: MenuProps["onClick"] = ({ key }) => {
  router.push(String(key))
}
</script>

<style scoped lang="scss">
/* 让设置菜单透明化，融入磨砂玻璃面板 */
.setting-menu {
  background: transparent;

  &.ant-menu-inline {
    border-inline-end: none;
  }

  :deep(.ant-menu-item) {
    border-radius: 10px;
    margin-inline: 0;
    width: 100%;
  }
}
</style>
