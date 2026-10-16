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

        <a-divider />

        <!-- 磨砂玻璃透明度（拖动即时生效） -->
        <section>
            <div class="mb-1 text-sm font-medium">{{ t("settings.appearance.glass") }}</div>
            <p class="mb-3 text-xs opacity-60">{{ t("settings.appearance.glassTip") }}</p>

            <div class="mb-4">
                <div class="mb-1 flex items-center justify-between text-xs">
                    <span>{{ t("settings.appearance.cardTransparency") }}</span>
                    <span class="font-medium tabular-nums">{{ appStore.cardTransparency }}%</span>
                </div>
                <a-slider v-model:value="appStore.cardTransparency" :min="0" :max="100" />
            </div>

            <div class="mb-4">
                <div class="mb-1 flex items-center justify-between text-xs">
                    <span>{{ t("settings.appearance.modalTransparency") }}</span>
                    <span class="font-medium tabular-nums">{{ appStore.modalTransparency }}%</span>
                </div>
                <a-slider v-model:value="appStore.modalTransparency" :min="0" :max="100" />
            </div>

            <!-- 实时预览：两种磨砂效果叠加在彩色底图上，拖动滑块即时可见 -->
            <div class="glass-preview" aria-hidden="true">
                <div class="glass-preview-card"></div>
                <div class="glass-preview-modal"></div>
            </div>
            <p class="mt-2 text-xs opacity-60">{{ t("settings.appearance.previewTip") }}</p>
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

/* 透明度实时预览：纯色底图（暖调中性色，不用渐变）+ 卡片 / 弹窗两种磨砂效果 */
.glass-preview {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 14px;
    border-radius: 12px;
    background-color: #a09adf;
}

.dark .glass-preview {
    background-color: #a09adf;
}

.glass-preview-card,
.glass-preview-modal {
    border: 1px solid rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(10px) saturate(160%);
    -webkit-backdrop-filter: blur(10px) saturate(160%);
}

.glass-preview-card {
    height: 48px;
    border-radius: 12px;
    background: rgba(255, 255, 255, var(--app-card-alpha));
}

.glass-preview-modal {
    height: 48px;
    border-radius: 10px;
    background: rgba(255, 255, 255, var(--app-modal-alpha));
}

.dark .glass-preview-card {
    background: rgba(30, 41, 59, var(--app-card-alpha));
}

.dark .glass-preview-modal {
    background: rgba(30, 41, 59, var(--app-modal-alpha));
}
</style>
