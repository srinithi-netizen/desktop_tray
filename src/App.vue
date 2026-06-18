<template>
  <div class="app-container">
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
</template>

<script setup>
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import ClientSelector from './components/ClientSelector.vue'
import FileSelector from './components/FileSelector.vue'
import UploadHistorySection from './components/UploadHistorySection.vue'
import NetworkStatus from './components/NetworkStatus.vue'

const uploads = ref(JSON.parse(localStorage.getItem('uploads') || '[]'))

function onUploadSaved(newUpload) {
  uploads.value.push(newUpload)
}

function onUploadDeleted(id) {
  uploads.value = uploads.value.filter(upload => upload.id !== id)
  localStorage.setItem('uploads', JSON.stringify(uploads.value))
}

// getCurrentWindow() returns a reference to the WebviewWindow this
// Vue app is running inside. .hide() sends a message over Tauri's IPC
// bridge to the Rust side, which calls window.hide() natively.
// The process keeps running — only the window becomes invisible.
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