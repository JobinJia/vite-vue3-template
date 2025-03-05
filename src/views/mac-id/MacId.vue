<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'

// 创建消息实例
const message = useMessage()

// 存储MAC地址信息
const originalMacAddress = ref<string>('')
const currentMacAddress = ref<string>('')
const isLoading = ref<boolean>(false)
const isChanging = ref<boolean>(false)
const isRestoring = ref<boolean>(false)
const errorMessage = ref<string>('')
const autoRestoreOnReboot = ref<boolean>(false)

// 获取原始MAC地址
async function fetchOriginalMacAddress() {
  isLoading.value = true
  errorMessage.value = ''

  try {
    // 调用Tauri后端获取MAC地址
    const macAddress = await invoke('get_mac_address')
    originalMacAddress.value = macAddress as string
    currentMacAddress.value = macAddress as string

    // 获取自动还原设置
    try {
      const autoRestore = await invoke('get_auto_restore_setting')
      autoRestoreOnReboot.value = autoRestore as boolean
    } catch (error) {
      console.error('获取自动还原设置失败:', error)
      // 默认为false
      autoRestoreOnReboot.value = false
    }
  } catch (error) {
    console.error('获取MAC地址失败:', error)
    errorMessage.value = `获取MAC地址失败: ${error}`
    message.error('获取MAC地址失败')
    // 模拟数据，实际应用中应删除
    originalMacAddress.value = '00:1A:2B:3C:4D:5E'
    currentMacAddress.value = '00:1A:2B:3C:4D:5E'
  } finally {
    isLoading.value = false
  }
}

// 生成随机MAC地址
function generateRandomMacAddress(): string {
  const hexDigits = '0123456789ABCDEF'
  let macAddress = ''

  for (let i = 0; i < 6; i++) {
    let part = ''
    for (let j = 0; j < 2; j++) {
      part += hexDigits.charAt(Math.floor(Math.random() * hexDigits.length))
    }
    macAddress += (i === 0 ? '' : ':') + part
  }

  return macAddress
}

// 修改MAC地址
async function changeMacAddress() {
  isChanging.value = true
  errorMessage.value = ''

  try {
    const newMacAddress = generateRandomMacAddress()

    // 调用Tauri后端修改MAC地址
    await invoke('change_mac_address', { macAddress: newMacAddress })
    message.success('MAC地址修改成功')

    // 更新当前MAC地址
    currentMacAddress.value = newMacAddress
  } catch (error) {
    console.error('修改MAC地址失败:', error)
    errorMessage.value = `修改MAC地址失败: ${error}`
    message.error('修改MAC地址失败')
  } finally {
    isChanging.value = false
  }
}

// 还原MAC地址
async function restoreMacAddress() {
  isRestoring.value = true
  errorMessage.value = ''

  try {
    // 调用Tauri后端还原MAC地址
    await invoke('restore_mac_cmd')
    message.success('MAC地址已还原')

    // 重新获取MAC地址
    const macAddress = await invoke('get_mac_address')
    currentMacAddress.value = macAddress as string
  } catch (error) {
    console.error('还原MAC地址失败:', error)
    errorMessage.value = `还原MAC地址失败: ${error}`
    message.error('还原MAC地址失败')
  } finally {
    isRestoring.value = false
  }
}

// 更新自动还原设置
async function updateAutoRestoreSetting(value: boolean) {
  try {
    await invoke('set_auto_restore_setting', { autoRestore: value })
    message.success(value ? '已开启重启自动还原' : '已关闭重启自动还原')
    autoRestoreOnReboot.value = value
  } catch (error) {
    console.error('更新自动还原设置失败:', error)
    message.error('更新设置失败')
    // 恢复原值
    autoRestoreOnReboot.value = !value
  }
}

// 组件挂载时获取MAC地址
onMounted(() => {
  fetchOriginalMacAddress()
})
</script>

<template>
  <div class="mac-id-container">
    <n-card title="MAC地址管理" class="mac-card">
      <template #footer>
        <n-space>
          <n-button type="primary" :loading="isChanging" @click="changeMacAddress">
            随机修改MAC地址
          </n-button>
          <n-button :loading="isRestoring" @click="restoreMacAddress">
            还原MAC地址
          </n-button>
        </n-space>
      </template>

      <n-space vertical size="large">
        <n-spin :show="isLoading" description="正在获取MAC地址...">
          <div class="mac-info">
            <n-descriptions bordered>
              <n-descriptions-item label="原始MAC地址">
                <n-tag type="primary">
                  {{ originalMacAddress }}
                </n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="当前MAC地址">
                <n-tag
                  :type="currentMacAddress === originalMacAddress ? 'success' : 'warning'"
                >
                  {{ currentMacAddress }}
                </n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="状态">
                <n-tag :type="currentMacAddress === originalMacAddress ? 'success' : 'warning'">
                  {{ currentMacAddress === originalMacAddress ? '原始地址' : '已修改' }}
                </n-tag>
              </n-descriptions-item>
            </n-descriptions>
          </div>
        </n-spin>

        <n-card title="设置" size="small">
          <n-space vertical>
            <n-space align="center">
              <span>重启电脑时自动还原MAC地址：</span>
              <n-switch v-model:value="autoRestoreOnReboot" @update:value="updateAutoRestoreSetting" />
            </n-space>
            <n-text depth="3" size="small">
              开启此选项后，系统重启时将自动还原为原始MAC地址。这有助于避免长期使用修改后的MAC地址可能带来的问题。
            </n-text>
          </n-space>
        </n-card>

        <n-alert v-if="errorMessage" type="error" :title="errorMessage" />

        <div class="mac-info-help">
          <n-collapse>
            <n-collapse-item title="什么是MAC地址？" name="1">
              <p>MAC地址（Media Access Control Address）是一个用于识别网络设备的唯一标识符，由48位二进制数字组成，通常表示为12个十六进制数字，分为6组，每组2个十六进制数字，组之间用冒号或连字符分隔。</p>
            </n-collapse-item>
            <n-collapse-item title="为什么要修改MAC地址？" name="2">
              <p>修改MAC地址可能有多种原因，包括：</p>
              <ul>
                <li>绕过基于MAC地址的网络访问控制</li>
                <li>解决网络设备冲突问题</li>
                <li>增强隐私保护，避免被跟踪</li>
                <li>测试网络配置和安全性</li>
              </ul>
              <p>请注意，在某些情况下修改MAC地址可能违反网络使用政策，请确保您的操作符合相关法律法规。</p>
            </n-collapse-item>
          </n-collapse>
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<style scoped>
.mac-id-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.mac-card {
  width: 100%;
}

.mac-info {
  margin-bottom: 20px;
}

.mac-info-help {
  margin-top: 20px;
}
</style>
