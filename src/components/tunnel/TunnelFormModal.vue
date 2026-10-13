<template>
  <a-modal
    :open="open"
    :title="isEdit ? t('tunnel.form.editTitle') : t('tunnel.form.addTitle')"
    :confirm-loading="saving"
    :mask-closable="false"
    :width="640"
    destroy-on-close
    :ok-text="t('common.save')"
    :cancel-text="t('common.cancel')"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <a-form ref="formRef" :model="form" :rules="rules" :disabled="saving" layout="vertical">
      <!-- 基础信息 -->
      <a-form-item :label="t('tunnel.name')" name="name">
        <a-input
          v-model:value="form.name"
          :placeholder="t('tunnel.namePlaceholder')"
          :maxlength="50"
          allow-clear
        />
      </a-form-item>

      <a-row :gutter="12">
        <a-col :span="12">
          <a-form-item :label="t('tunnel.type')" name="type">
            <a-select v-model:value="form.type" :options="typeOptions" />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item :label="t('tunnel.localIp')" name="localIp">
            <a-input
              v-model:value="form.localIp"
              :placeholder="t('tunnel.localIpPlaceholder')"
              allow-clear
            />
          </a-form-item>
        </a-col>
      </a-row>

      <a-row :gutter="12">
        <a-col :span="12">
          <a-form-item :label="t('tunnel.localPort')" name="localPort">
            <a-input-number v-model:value="form.localPort" :min="1" :max="65535" class="w-full" />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            :label="t('tunnel.remotePort')"
            name="remotePort"
            :tooltip="t('tunnel.remotePortTip')"
          >
            <a-input-number
              v-model:value="form.remotePort"
              :min="1"
              :max="65535"
              class="w-full"
              :placeholder="showRemotePort ? '' : t('tunnel.remotePortTip')"
            />
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item :label="t('tunnel.remark')" name="remark">
        <a-textarea
          v-model:value="form.remark"
          :rows="2"
          :maxlength="200"
          show-count
          :placeholder="t('tunnel.remarkPlaceholder')"
        />
      </a-form-item>

      <a-row :gutter="12">
        <a-col :span="8">
          <a-form-item :label="t('tunnel.sort')" name="sort">
            <a-input-number v-model:value="form.sort" class="w-full" />
          </a-form-item>
        </a-col>
        <a-col :span="8">
          <a-form-item :label="t('tunnel.pinned')" name="pinned">
            <a-switch v-model:checked="form.pinned" />
          </a-form-item>
        </a-col>
        <a-col :span="8">
          <a-form-item
            :label="t('tunnel.enabled')"
            name="enabled"
            :tooltip="t('tunnel.enabledTip')"
          >
            <a-switch v-model:checked="form.enabled" />
          </a-form-item>
        </a-col>
      </a-row>

      <!-- 高级设置（折叠面板） -->
      <a-collapse v-model:active-key="activeKeys" ghost>
        <a-collapse-panel key="advanced" :header="t('tunnel.advanced')">
          <!-- 域名 / 密钥（按代理类型展示） -->
          <a-row v-if="showDomain" :gutter="12">
            <a-col :span="showSubdomain ? 12 : 24">
              <a-form-item :label="t('tunnel.customDomains')" name="customDomains">
                <a-select
                  v-model:value="form.customDomains"
                  mode="tags"
                  :open="false"
                  :token-separators="[',']"
                  :placeholder="t('tunnel.customDomainsPlaceholder')"
                />
              </a-form-item>
            </a-col>
            <a-col v-if="showSubdomain" :span="12">
              <a-form-item :label="t('tunnel.subdomain')" name="subdomain">
                <a-input
                  v-model:value="form.subdomain"
                  :placeholder="t('tunnel.subdomainPlaceholder')"
                  allow-clear
                />
              </a-form-item>
            </a-col>
          </a-row>

          <a-form-item v-if="showLocations" :label="t('tunnel.locations')" name="locations">
            <a-select
              v-model:value="form.locations"
              mode="tags"
              :open="false"
              :token-separators="[',']"
              :placeholder="t('tunnel.locationsPlaceholder')"
            />
          </a-form-item>

          <a-form-item v-if="showSecret" :label="t('tunnel.secretKey')" name="secretKey">
            <a-input-password
              v-model:value="form.secretKey"
              :placeholder="t('tunnel.secretKeyPlaceholder')"
              allow-clear
            />
          </a-form-item>

          <!-- 传输配置 -->
          <a-row :gutter="12">
            <a-col :span="12">
              <a-form-item :label="t('tunnel.useEncryption')" name="useEncryption">
                <a-switch v-model:checked="form.useEncryption" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item :label="t('tunnel.useCompression')" name="useCompression">
                <a-switch v-model:checked="form.useCompression" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="12">
            <a-col :span="12">
              <a-form-item :label="t('tunnel.bandwidthLimit')" name="bandwidthLimit">
                <a-input
                  v-model:value="form.bandwidthLimit"
                  :placeholder="t('tunnel.bandwidthLimitPlaceholder')"
                  allow-clear
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item
                :label="t('tunnel.proxyProtocolVersion')"
                name="proxyProtocolVersion"
              >
                <a-select
                  v-model:value="form.proxyProtocolVersion"
                  :options="proxyProtocolOptions"
                />
              </a-form-item>
            </a-col>
          </a-row>

          <!-- 健康检查 -->
          <a-divider orientation="left" class="!my-2 !text-xs">
            {{ t("tunnel.healthCheck") }}
          </a-divider>

          <a-row :gutter="12">
            <a-col :span="12">
              <a-form-item :label="t('tunnel.healthCheckType')" name="healthCheckType">
                <a-select
                  v-model:value="form.healthCheckType"
                  :options="healthCheckTypeOptions"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item :label="t('tunnel.healthCheckTimeout')" name="healthCheckTimeout">
                <a-input-number v-model:value="form.healthCheckTimeout" :min="1" class="w-full" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="12">
            <a-col :span="12">
              <a-form-item :label="t('tunnel.healthCheckInterval')" name="healthCheckInterval">
                <a-input-number v-model:value="form.healthCheckInterval" :min="1" class="w-full" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item :label="t('tunnel.healthCheckMaxFailed')" name="healthCheckMaxFailed">
                <a-input-number v-model:value="form.healthCheckMaxFailed" :min="1" class="w-full" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-form-item :label="t('tunnel.healthCheckPath')" name="healthCheckPath">
            <a-input
              v-model:value="form.healthCheckPath"
              :placeholder="t('tunnel.healthCheckPathPlaceholder')"
              allow-clear
            />
          </a-form-item>

          <!-- 负载均衡 -->
          <a-divider orientation="left" class="!my-2 !text-xs">
            {{ t("tunnel.loadBalancer") }}
          </a-divider>

          <a-row :gutter="12">
            <a-col :span="12">
              <a-form-item :label="t('tunnel.loadBalancerGroup')" name="loadBalancerGroup">
                <a-input
                  v-model:value="form.loadBalancerGroup"
                  :placeholder="t('tunnel.loadBalancerGroupPlaceholder')"
                  allow-clear
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item :label="t('tunnel.loadBalancerGroupKey')" name="loadBalancerGroupKey">
                <a-input v-model:value="form.loadBalancerGroupKey" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
        </a-collapse-panel>
      </a-collapse>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { message } from "ant-design-vue"
import type { FormInstance, FormProps } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { createTunnel, updateTunnel } from "@/api/tunnel"
import {
  TUNNEL_TYPES,
  TYPES_WITH_DOMAIN,
  TYPES_WITH_REMOTE_PORT,
  TYPES_WITH_SECRET,
  type Tunnel,
  type TunnelPayload,
  type TunnelType,
} from "@/types/tunnel"

const props = defineProps<{
  open: boolean
  /** 编辑时传入，新增时为 null */
  tunnel?: Tunnel | null
}>()

const emit = defineEmits<{
  "update:open": [value: boolean]
  saved: []
}>()

const { t } = useI18n()

interface FormState {
  name: string
  type: TunnelType
  localIp: string
  localPort: number | undefined
  remotePort: number | undefined
  customDomains: string[]
  subdomain: string
  locations: string[]
  secretKey: string
  useEncryption: boolean
  useCompression: boolean
  bandwidthLimit: string
  proxyProtocolVersion: string
  healthCheckType: string
  healthCheckTimeout: number | undefined
  healthCheckMaxFailed: number | undefined
  healthCheckInterval: number | undefined
  healthCheckPath: string
  loadBalancerGroup: string
  loadBalancerGroupKey: string
  remark: string
  sort: number
  pinned: boolean
  enabled: boolean
}

const formRef = ref<FormInstance>()
const saving = ref(false)
const activeKeys = ref<string[]>([])

const form = reactive<FormState>({
  name: "",
  type: "tcp",
  localIp: "127.0.0.1",
  localPort: undefined,
  remotePort: undefined,
  customDomains: [],
  subdomain: "",
  locations: [],
  secretKey: "",
  useEncryption: false,
  useCompression: false,
  bandwidthLimit: "",
  proxyProtocolVersion: "",
  healthCheckType: "",
  healthCheckTimeout: undefined,
  healthCheckMaxFailed: undefined,
  healthCheckInterval: undefined,
  healthCheckPath: "",
  loadBalancerGroup: "",
  loadBalancerGroupKey: "",
  remark: "",
  sort: 0,
  pinned: false,
  enabled: false,
})

const isEdit = computed(() => props.tunnel?.id != null)

/** 依据代理类型决定字段的显隐 */
const showRemotePort = computed(() => TYPES_WITH_REMOTE_PORT.includes(form.type))
const showDomain = computed(() => TYPES_WITH_DOMAIN.includes(form.type))
const showSubdomain = computed(() => form.type === "http" || form.type === "https")
const showLocations = computed(() => form.type === "http")
const showSecret = computed(() => TYPES_WITH_SECRET.includes(form.type))

const typeOptions = computed(() =>
  TUNNEL_TYPES.map((value) => ({ value, label: t(`tunnel.types.${value}`) })),
)

const proxyProtocolOptions = computed(() => [
  { value: "", label: t("tunnel.proxyProtocolVersionDefault") },
  { value: "v1", label: "v1" },
  { value: "v2", label: "v2" },
])

const healthCheckTypeOptions = computed(() => [
  { value: "", label: t("tunnel.healthCheckTypeDefault") },
  { value: "tcp", label: "TCP" },
  { value: "http", label: "HTTP" },
])

/** 是否为合法的 IPv4 或域名 */
function isValidHost(value: string): boolean {
  const host = value.trim()
  const ipv4 = /^(\d{1,3}\.){3}\d{1,3}$/
  if (ipv4.test(host)) {
    return host.split(".").every((part) => Number(part) >= 0 && Number(part) <= 255)
  }
  const hostname =
    /^(?=.{1,253}$)([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
  return hostname.test(host)
}

/** 端口范围校验器 */
async function portValidator(_rule: unknown, value: number | null) {
  if (value != null && (value < 1 || value > 65535)) {
    throw new Error(t("tunnel.form.portRange"))
  }
}

// 依据代理类型动态调整校验规则（远程端口仅 tcp / udp 必填）
const rules = computed<FormProps["rules"]>(() => ({
  name: [{ required: true, message: t("tunnel.form.nameRequired"), trigger: "blur" }],
  type: [{ required: true, message: t("tunnel.form.typeRequired"), trigger: "change" }],
  localIp: [
    { required: true, message: t("tunnel.form.localIpRequired"), trigger: "blur" },
    {
      trigger: "blur",
      validator: async (_rule: unknown, value: string) => {
        if (value && !isValidHost(value)) {
          throw new Error(t("tunnel.form.localIpInvalid"))
        }
      },
    },
  ],
  localPort: [
    { required: true, message: t("tunnel.form.localPortRequired"), trigger: "change" },
    { trigger: "change", validator: portValidator },
  ],
  remotePort: showRemotePort.value
    ? [
        { required: true, message: t("tunnel.form.remotePortRequired"), trigger: "change" },
        { trigger: "change", validator: portValidator },
      ]
    : [],
}))

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 英文逗号分隔字符串 → 数组 */
function splitList(value?: string | null): string[] {
  if (!value) return []
  return value
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean)
}

/** 数组 → 英文逗号分隔字符串（空数组返回 null） */
function joinList(value: string[]): string | null {
  const list = value.map((item) => item.trim()).filter(Boolean)
  return list.length ? list.join(",") : null
}

/** 空字符串转 null */
function textOrNull(value: string): string | null {
  return value.trim() || null
}

/** 根据编辑对象或默认值重置表单 */
function resetForm() {
  const tunnel = props.tunnel
  form.name = tunnel?.name ?? ""
  form.type = tunnel?.type ?? "tcp"
  form.localIp = tunnel?.local_ip ?? "127.0.0.1"
  form.localPort = tunnel?.local_port ?? undefined
  form.remotePort = tunnel?.remote_port ?? undefined
  form.customDomains = splitList(tunnel?.custom_domains)
  form.subdomain = tunnel?.subdomain ?? ""
  form.locations = splitList(tunnel?.locations)
  form.secretKey = tunnel?.secret_key ?? ""
  form.useEncryption = tunnel?.use_encryption ?? false
  form.useCompression = tunnel?.use_compression ?? false
  form.bandwidthLimit = tunnel?.bandwidth_limit ?? ""
  form.proxyProtocolVersion = tunnel?.proxy_protocol_version ?? ""
  form.healthCheckType = tunnel?.health_check_type ?? ""
  form.healthCheckTimeout = tunnel?.health_check_timeout ?? undefined
  form.healthCheckMaxFailed = tunnel?.health_check_max_failed ?? undefined
  form.healthCheckInterval = tunnel?.health_check_interval ?? undefined
  form.healthCheckPath = tunnel?.health_check_path ?? ""
  form.loadBalancerGroup = tunnel?.load_balancer_group ?? ""
  form.loadBalancerGroupKey = tunnel?.load_balancer_group_key ?? ""
  form.remark = tunnel?.remark ?? ""
  form.sort = tunnel?.sort ?? 0
  form.pinned = tunnel?.pinned ?? false
  form.enabled = tunnel?.enabled ?? false
}

watch(
  () => props.open,
  (open) => {
    if (!open) return
    activeKeys.value = []
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
    const payload: TunnelPayload = {
      id: props.tunnel?.id,
      name: form.name.trim(),
      type: form.type,
      local_ip: form.localIp.trim(),
      local_port: Number(form.localPort ?? 0),
      remote_port: showRemotePort.value ? (form.remotePort ?? null) : null,
      custom_domains: showDomain.value ? joinList(form.customDomains) : null,
      subdomain: showSubdomain.value ? textOrNull(form.subdomain) : null,
      locations: showLocations.value ? joinList(form.locations) : null,
      secret_key: showSecret.value ? textOrNull(form.secretKey) : null,
      use_encryption: form.useEncryption,
      use_compression: form.useCompression,
      bandwidth_limit: textOrNull(form.bandwidthLimit),
      proxy_protocol_version: form.proxyProtocolVersion || null,
      health_check_type: form.healthCheckType || null,
      health_check_timeout: form.healthCheckTimeout ?? null,
      health_check_max_failed: form.healthCheckMaxFailed ?? null,
      health_check_interval: form.healthCheckInterval ?? null,
      health_check_path: textOrNull(form.healthCheckPath),
      load_balancer_group: textOrNull(form.loadBalancerGroup),
      load_balancer_group_key: textOrNull(form.loadBalancerGroupKey),
      remark: textOrNull(form.remark),
      sort: Number(form.sort ?? 0),
      pinned: form.pinned,
      enabled: form.enabled,
    }

    if (isEdit.value) {
      await updateTunnel(payload)
    } else {
      await createTunnel(payload)
    }

    message.success(t("tunnel.saveSuccess"))
    emit("saved")
    emit("update:open", false)
  } catch (e) {
    message.error(errMsg(e))
  } finally {
    saving.value = false
  }
}
</script>
