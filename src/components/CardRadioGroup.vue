<template>
  <!-- 卡片式单选组：标题 + 描述纵向排列，选中时整卡高亮。关闭弹窗与「设置 - 常规」共用 -->
  <a-radio-group v-model:value="model" class="card-radio-group" :disabled="disabled">
    <a-radio
      v-for="option in options"
      :key="String(option.value)"
      :value="option.value"
      class="card-radio"
    >
      <span class="card-radio-title">{{ option.label }}</span>
      <span v-if="option.description" class="card-radio-desc">{{ option.description }}</span>
    </a-radio>
  </a-radio-group>
</template>

<script setup lang="ts" generic="T extends string | number">
withDefaults(
  defineProps<{
    options: { value: T; label: string; description?: string }[]
    disabled?: boolean
  }>(),
  { disabled: false },
)

const model = defineModel<T>({ required: true })
</script>

<style scoped lang="scss">
.card-radio-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  width: 100%;
}

.card-radio {
  display: flex;
  align-items: flex-start;
  width: 100%;
  margin-inline-end: 0;
  padding: 7px 10px;
  border-radius: 8px;
  border: 1px solid rgba(15, 23, 42, 0.1);
  transition: border-color 0.2s ease, background-color 0.2s ease;

  &:hover {
    border-color: rgba(99, 102, 241, 0.5);
  }

  // 选中态高亮
  &.ant-radio-wrapper-checked {
    border-color: #6366f1;
    background-color: rgba(99, 102, 241, 0.08);
  }

  :deep(.ant-radio) {
    margin-top: 1px;
  }

  :deep(> span:last-child) {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding-inline-end: 0;
  }
}

.dark .card-radio {
  border-color: rgba(255, 255, 255, 0.12);

  &.ant-radio-wrapper-checked {
    border-color: #6366f1;
    background-color: rgba(99, 102, 241, 0.16);
  }
}

.card-radio-title {
  font-size: 13px;
  font-weight: 500;
  line-height: 1.5;
}

.card-radio-desc {
  font-size: 11.5px;
  line-height: 1.35;
  opacity: 0.6;
}
</style>
