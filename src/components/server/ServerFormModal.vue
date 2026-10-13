<template>
  <a-modal
    :open="open"
    :title="isEdit ? t('server.form.editTitle') : t('server.form.addTitle')"
    :confirm-loading="saving"
    :mask-closable="false"
    destroy-on-close
    :ok-text="t('common.save')"
    :cancel-text="t('common.cancel')"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <a-form ref="formRef" :model="form" :rules="rules" :disabled="saving" layout="vertical">
      <a-form-item :label="t('server.name')" name="name">
        <a-input
          v-model:value="form.name"
          :placeholder="t('server.namePlaceholder')"
          :maxlength="50"
          allow-clear
        />
      </a-form-item>

      <a-row :gutter="12">
        <a-col :span="16">
          <a-form-item :label="t('server.addr')" name="addr">
            <a-input
              v-model:value="form.addr"
              :placeholder="t('server.addrPlaceholder')"
              allow-clear
            />
          </a-form-item>
        </a-col>
        <a-col :span="8">
          <a-form-item :label="t('server.port')" name="port">
            <a-input-number v-model:value="form.port" :min="1" :max="65535" class="w-full" />
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item :label="t('server.token')" name="token">
        <a-input-password
          v-model:value="form.token"
          :placeholder="t('server.tokenPlaceholder')"
          allow-clear
        />
      </a-form-item>

      <a-row :gutter="12">
        <a-col :span="12">
          <a-form-item :label="t('server.protocol')" name="protocol">
            <a-select v-model:value="form.protocol" :options="protocolOptions" />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item :label="t('server.tls')" name="tls_enable">
            <a-switch v-model:checked="form.tls_enable" />
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item :label="t('server.remark')" name="remark">
        <a-textarea
          v-model:value="form.remark"
          :rows="2"
          :maxlength="200"
          show-count
          :placeholder="t('server.remarkPlaceholder')"
        />
      </a-form-item>

      <a-row :gutter="12">
        <a-col :span="8">
          <a-form-item :label="t('server.sort')" name="sort">
            <a-input-number v-model:value="form.sort" class="w-full" />
          </a-form-item>
        </a-col>
        <a-col :span="8">
          <a-form-item :label="t('server.pinned')" name="pinned">
            <a-switch v-model:checked="form.pinned" />
          </a-form-item>
        </a-col>
        <a-col :span="8">
          <a-form-item
            :label="t('server.enabled')"
            name="enabled"
            :tooltip="t('server.enabledTip')"
          >
            <a-switch v-model:checked="form.enabled" />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { message } from "ant-design-vue"
import type { FormInstance, FormProps } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { createServer, updateServer } from "@/api/server"
import {
  SERVER_PROTOCOLS,
  type Server,
  type ServerPayload,
  type ServerProtocol,
} from "@/types/server"

const props = defineProps<{
  open: boolean
  /** 编辑时传入，新增时为 null */
  server?: Server | null
}>()

const emit = defineEmits<{
  "update:open": [value: boolean]
  saved: []
}>()

const { t } = useI18n()

interface FormState {
  name: string
  addr: string
  port: number
  token: string
  protocol: ServerProtocol
  tls_enable: boolean
  remark: string
  sort: number
  pinned: boolean
  enabled: boolean
}

const formRef = ref<FormInstance>()
const saving = ref(false)

const protocolOptions = SERVER_PROTOCOLS

const form = reactive<FormState>({
  name: "",
  addr: "",
  port: 7000,
  token: "",
  protocol: "tcp",
  tls_enable: false,
  remark: "",
  sort: 0,
  pinned: false,
  enabled: false,
})

const isEdit = computed(() => props.server?.id != null)

/** 是否为合法的 IPv4 或域名 */
function isValidAddr(value: string): boolean {
  const addr = value.trim()
  const ipv4 = /^(\d{1,3}\.){3}\d{1,3}$/
  if (ipv4.test(addr)) {
    return addr.split(".").every((part) => Number(part) >= 0 && Number(part) <= 255)
  }
  const hostname = /^(?=.{1,253}$)([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
  return hostname.test(addr)
}

const rules: FormProps["rules"] = {
  name: [{ required: true, message: t("server.form.nameRequired"), trigger: "blur" }],
  addr: [
    { required: true, message: t("server.form.addrRequired"), trigger: "blur" },
    {
      trigger: "blur",
      validator: async (_rule, value: string) => {
        if (value && !isValidAddr(value)) {
          throw new Error(t("server.form.addrInvalid"))
        }
      },
    },
  ],
  port: [
    { required: true, message: t("server.form.portRequired"), trigger: "change" },
    {
      trigger: "change",
      validator: async (_rule, value: number | null) => {
        if (value != null && (value < 1 || value > 65535)) {
          throw new Error(t("server.form.portRange"))
        }
      },
    },
  ],
}

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 根据编辑对象或默认值重置表单 */
function resetForm() {
  const server = props.server
  form.name = server?.name ?? ""
  form.addr = server?.addr ?? ""
  form.port = server?.port ?? 7000
  form.token = server?.token ?? ""
  form.protocol = server?.protocol ?? "tcp"
  form.tls_enable = server?.tls_enable ?? false
  form.remark = server?.remark ?? ""
  form.sort = server?.sort ?? 0
  form.pinned = server?.pinned ?? false
  form.enabled = server?.enabled ?? false
}

watch(
  () => props.open,
  (open) => {
    if (!open) return
    resetForm()
    nextTick(() => formRef.value?.clearValidate())
  },
)

function handleCancel() {
  emit("update:open", false)
}

async function handleOk() {
  try {
    await formRef.value?.validate()
  } catch {
    return
  }

  saving.value = true
  try {
    const payload: ServerPayload = {
      id: props.server?.id,
      name: form.name.trim(),
      addr: form.addr.trim(),
      port: Number(form.port ?? 7000),
      token: form.token.trim() || null,
      protocol: form.protocol,
      tls_enable: form.tls_enable,
      remark: form.remark.trim() || null,
      sort: Number(form.sort ?? 0),
      pinned: form.pinned,
      enabled: form.enabled,
    }

    if (isEdit.value) {
      await updateServer(payload)
    } else {
      await createServer(payload)
    }

    message.success(t("server.saveSuccess"))
    emit("saved")
    emit("update:open", false)
  } catch (e) {
    message.error(errMsg(e))
  } finally {
    saving.value = false
  }
}
</script>
