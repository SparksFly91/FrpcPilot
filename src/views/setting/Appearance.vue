<template>
    <div class="max-w-2xl">
        <h2 class="mb-1 text-lg font-semibold">{{ t("settings.appearance.title") }}</h2>
        <p class="mb-6 text-xs opacity-60">{{ t("settings.appearance.subtitle") }}</p>

        <!-- 主题模式 -->
        <section class="mb-6">
            <div class="mb-2 text-sm font-medium">{{ t("settings.appearance.themeMode") }}</div>
            <a-segmented v-model:value="themeMode" :options="themeOptions" block />
            <p class="mt-2 text-xs opacity-60">{{ t("settings.appearance.themeModeTip") }}</p>
        </section>

        <a-divider />

        <!-- 背景图片 -->
        <section>
            <div class="mb-1 text-sm font-medium">{{ t("settings.appearance.background") }}</div>
            <p class="mb-3 text-xs opacity-60">{{ t("settings.appearance.backgroundTip") }}</p>

            <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4">
                <a-card v-for="bg in backgroundOptions" :key="bg.key" hoverable size="small"
                    class="bg-card overflow-hidden" :class="{ 'bg-card--active': appStore.background === bg.key }"
                    :body-style="{ padding: '8px 10px' }" @click="selectBackground(bg.key)">
                    <template #cover>
                        <div v-if="bg.url" class="bg-thumb" :style="{ backgroundImage: `url('${bg.url}')` }" />
                        <div v-else class="bg-thumb bg-thumb--none" />
                    </template>

                    <div class="flex items-center justify-between gap-1 text-xs">
                        <span class="truncate">{{ t(bg.label) }}</span>
                        <CheckCircleFilled v-if="appStore.background === bg.key" class="shrink-0 text-[#6366f1]" />
                    </div>
                </a-card>
            </div>
        </section>
    </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { BulbFilled, BulbOutlined, CheckCircleFilled } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import { backgroundOptions, type BackgroundKey } from "@/config/backgrounds"
import { useAppStore } from "@/stores/app"

const { t } = useI18n()
const appStore = useAppStore()

/** 主题模式：与 store 中的 isDark 双向绑定 */
const themeMode = computed({
    get: () => (appStore.isDark ? "dark" : "light"),
    set: (value: string | number) => {
        appStore.isDark = value === "dark"
    },
})

const themeOptions = computed(() => [
    {
        value: "light",
        label: h("span", { class: "inline-flex items-center gap-2" }, [
            h(BulbOutlined),
            t("settings.appearance.light"),
        ]),
    },
    {
        value: "dark",
        label: h("span", { class: "inline-flex items-center gap-2" }, [
            h(BulbFilled),
            t("settings.appearance.dark"),
        ]),
    },
])

function selectBackground(key: BackgroundKey) {
    appStore.setBackground(key)
}
</script>

<style scoped lang="scss">
.bg-card {
    cursor: pointer;
    border-color: rgba(15, 23, 42, 0.08);
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.dark .bg-card {
    border-color: rgba(255, 255, 255, 0.1);
}

.bg-card--active {
    border-color: #6366f1 !important;
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.35);
}

/* 缩略图：保持比例，不拉伸 */
.bg-thumb {
    height: 72px;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
}

.bg-thumb--none {
    background-image: repeating-linear-gradient(45deg,
            rgba(148, 163, 184, 0.25) 0 8px,
            transparent 8px 16px);
}
</style>
