<template>
  <div>

    <!-- Show activation screen if not yet activated -->
    <ActivationScreen v-if="!isActivated" @activated="onActivated" />

    <!-- Show main app after activation -->
    <div v-else class="app-container">
      <header class="app-header">
        <h1>FluxBooks Desktop Tray</h1>
        <NetworkStatus />
        <button class="tray-btn" @click="minimizeToTray">
          Minimize to Tray
        </button>
      </header>

      <main class="app-main">
        <ClientSelector />
        <FileSelector @upload-saved="onUploadSaved" />
        <UploadHistorySection
          :uploads="uploads"
          @upload-deleted="onUploadDeleted"
        />
      </main>
    </div>

  </div>
</template>

<script setup>
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import ActivationScreen from './components/ActivationScreen.vue'
import ClientSelector from './components/ClientSelector.vue'
import FileSelector from './components/FileSelector.vue'
import UploadHistorySection from './components/UploadHistorySection.vue'
import NetworkStatus from './components/NetworkStatus.vue'

// Check localStorage on launch — if session exists, skip activation screen
const isActivated = ref(!!localStorage.getItem('fluxbooks_session'))

const uploads = ref(JSON.parse(localStorage.getItem('uploads') || '[]'))

// Called by ActivationScreen when activation succeeds
function onActivated() {
  isActivated.value = true
}

function onUploadSaved(newUpload) {
  uploads.value.push(newUpload)
}

function onUploadDeleted(id) {
  uploads.value = uploads.value.filter(upload => upload.id !== id)
  localStorage.setItem('uploads', JSON.stringify(uploads.value))
}

async function minimizeToTray() {
  await getCurrentWindow().hide()
}
</script>

<style>
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: Arial, sans-serif;
  background-color: #f4f5f7;
}

.app-container {
  max-width: 480px;
  margin: 0 auto;
  padding: 24px;
}

.app-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  margin-bottom: 24px;
}

.app-header h1 {
  font-size: 20px;
  color: #2d2d2d;
  margin: 0;
}

.tray-btn {
  padding: 6px 14px;
  font-size: 12px;
  background-color: #f0f0f0;
  color: #444;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
}

.tray-btn:hover {
  background-color: #e0e0e0;
}

.app-main {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.card {
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
</style>