<template>
    <div class="max-w-2xl">
        <h2 class="mb-1 text-lg font-semibold">{{ t("settings.appearance.title") }}</h2>
        <p class="mb-6 text-xs opacity-60">{{ t("settings.appearance.subtitle") }}</p>

        <!-- 主题模式 -->
        <section class="mb-6">
            <div class="mb-2 text-sm font-medium">{{ t("settings.appearance.themeMode") }}</div>
            <a-segmented v-model:value="appStore.themeMode" :options="themeOptions" block />
            <p class="mt-2 text-xs opacity-60">{{ t("settings.appearance.themeModeTip") }}</p>
        </section>

        <a-divider />

        <section class="mb-6">
            <div class="mb-1 text-sm font-medium">{{ t("settings.appearance.backgroundOverlay") }}</div>
            <p class="mb-3 text-xs opacity-60">{{ t("settings.appearance.backgroundOverlayTip") }}</p>
            <div class="mb-1 flex items-center justify-between text-xs">
                <span>{{ t("settings.appearance.overlayTransparency") }}</span>
                <span class="font-medium tabular-nums">{{ appStore.backgroundOverlayTransparency }}%</span>
            </div>
            <a-slider v-model:value="appStore.backgroundOverlayTransparency" :min="0" :max="100" />
            <div class="overlay-preview" :style="{ '--preview-overlay-alpha': `${(100 - appStore.backgroundOverlayTransparency) / 100}` }" aria-hidden="true">
                <span>{{ t("settings.appearance.backgroundOverlayPreview") }}</span>
                <div class="overlay-preview-toolbar"><i /><i /><i /></div>
            </div>
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

                <a-card
                    hoverable
                    size="small"
                    class="bg-card overflow-hidden"
                    :class="{ 'bg-card--active': appStore.background === 'custom' }"
                    :body-style="{ padding: '8px 10px' }"
                    @click="selectCustomBackground"
                >
                    <template #cover>
                        <div
                            v-if="appStore.customBackgroundUrl"
                            class="bg-thumb"
                            :style="{ backgroundImage: `url('${appStore.customBackgroundUrl}')` }"
                        />
                        <div v-else class="bg-thumb bg-thumb--custom-empty">
                            <a-button type="link" :icon="h(PictureOutlined)" @click.stop="chooseCustomBackground">
                                {{ t("settings.appearance.chooseCustomBackground") }}
                            </a-button>
                        </div>
                    </template>

                    <div class="flex items-center justify-between gap-1 text-xs">
                        <span class="truncate">{{ t("settings.appearance.customBackground") }}</span>
                        <a-space :size="2">
                            <a-tooltip :title="t('settings.appearance.changeCustomBackground')">
                                <a-button
                                    type="text"
                                    size="small"
                                    :icon="h(UploadOutlined)"
                                    :aria-label="t('settings.appearance.changeCustomBackground')"
                                    @click.stop="chooseCustomBackground"
                                />
                            </a-tooltip>
                            <CheckCircleFilled v-if="appStore.background === 'custom'" class="shrink-0 text-[#6366f1]" />
                        </a-space>
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
import { BulbFilled, BulbOutlined, CheckCircleFilled, DesktopOutlined, PictureOutlined, UploadOutlined } from "@ant-design/icons-vue"
import { message } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { backgroundOptions, type BackgroundKey } from "@/config/backgrounds"
import { useAppStore } from "@/stores/app"

const { t } = useI18n()
const appStore = useAppStore()

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
    {
        value: "system",
        label: h("span", { class: "inline-flex items-center gap-2" }, [
            h(DesktopOutlined),
            t("settings.appearance.system"),
        ]),
    },
])

function selectBackground(key: BackgroundKey) {
    appStore.setBackground(key)
}

async function selectCustomBackground() {
    if (appStore.customBackgroundUrl) {
        appStore.setBackground("custom")
        return
    }
    await chooseCustomBackground()
}

async function chooseCustomBackground() {
    try {
        await appStore.chooseCustomBackground()
    } catch (error) {
        message.error(error instanceof Error ? error.message : String(error))
    }
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
    background-color: #e9edef;
}

.bg-thumb--custom-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #dfe5e4;
}

.dark .bg-thumb--custom-empty {
    background: #252f30;
}

.dark .bg-thumb--none {
    background-color: #151c1d;
}

.overlay-preview {
    position: relative;
    display: flex;
    height: 88px;
    align-items: flex-end;
    justify-content: space-between;
    overflow: hidden;
    padding: 12px;
    border: 1px solid rgba(15, 23, 42, 0.12);
    border-radius: 8px;
    background-color: #71817b;
    color: #14201d;
    font-size: 11px;
}

.overlay-preview::before {
    position: absolute;
    inset: 0;
    background: rgba(255, 255, 255, var(--preview-overlay-alpha));
    content: "";
    pointer-events: none;
}

.overlay-preview > * {
    position: relative;
}

.dark .overlay-preview {
    border-color: rgba(255, 255, 255, 0.12);
    background-color: #71817b;
    color: #f1f5f3;
}

.dark .overlay-preview::before {
    background: rgba(0, 0, 0, var(--preview-overlay-alpha));
}

.overlay-preview-toolbar {
    display: flex;
    gap: 5px;
}

.overlay-preview-toolbar i {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: currentColor;
    opacity: 0.7;
}

/* 透明度实时预览：中性纯色底图 + 卡片 / 弹窗两种磨砂效果 */
.glass-preview {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 14px;
    border-radius: 12px;
    background-color: #778880;
}

.dark .glass-preview {
    background-color: #778880;
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
