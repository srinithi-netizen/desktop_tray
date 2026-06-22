<template>
  <div class="card">
    <div class="section-header">
      <h2 class="section-title">Upload Queue</h2>
      <span class="count-badge">{{ uploads.length }} files</span>
    </div>

    <p v-if="uploads.length === 0" class="empty-message">No uploads yet</p>

    <ul v-else class="upload-list">
      <li
        v-for="upload in uploads"
        :key="upload.id"
        class="upload-item"
        :class="'item-' + upload.status"
      >
      <!-- CHUNK DETAILS TOGGLE -->
<div class="chunk-toggle">
  <button class="chunk-toggle-btn" @click="toggleChunks(upload.id)">
    {{ expandedId === upload.id ? '▲ Hide chunks' : '▼ Show chunks' }}
    ({{ upload.done_chunks }}/{{ upload.total_chunks }} done)
  </button>
</div>

<!-- CHUNK TABLE -->
<div v-if="expandedId === upload.id" class="chunk-table-wrap">
  <p v-if="loadingChunks" class="chunk-loading">Loading…</p>
  <table v-else class="chunk-table">
    <thead>
      <tr>
        <th>Chunk</th>
        <th>Size</th>
        <th>Status</th>
        <th>Sent At</th>
        <th>Error</th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="chunk in chunkMap[upload.id]"
        :key="chunk.chunk_index"
        :class="'chunk-' + chunk.status"
      >
        <td>{{ chunk.chunk_index + 1 }} / {{ chunk.total }}</td>
        <td>{{ formatSize(chunk.size_bytes) }}</td>
        <td>
          <span class="chunk-badge" :class="'cbadge-' + chunk.status">
            {{ chunkStatusLabel(chunk.status) }}
          </span>
        </td>
        <td>{{ chunk.sent_at ? formatTime(chunk.sent_at) : '—' }}</td>
        <td class="chunk-error">{{ chunk.error_msg || '—' }}</td>
      </tr>
    </tbody>
  </table>
</div>
        <!-- TOP ROW: icon + filename + delete -->
        <div class="upload-top">
          <span class="file-type-icon">{{ fileIcon(upload.file_name) }}</span>
          <div class="file-info">
            <p class="upload-filename">{{ upload.file_name }}</p>
            <p class="upload-id">ID: {{ upload.id.slice(0, 8) }}…</p>
          </div>
          <button class="delete-btn" @click="handleDelete(upload.id)" title="Remove">✕</button>
        </div>

        <!-- DETAILS ROW: size + date -->
        <div class="detail-row">
          <div class="detail-item">
            <span class="detail-label">Size</span>
            <span class="detail-value">{{ formatSize(upload.file_size) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">Queued</span>
            <span class="detail-value">{{ formatDate(upload.queued_at) }}</span>
          </div>
          <div v-if="upload.uploaded_at" class="detail-item">
            <span class="detail-label">Uploaded</span>
            <span class="detail-value">{{ formatDate(upload.uploaded_at) }}</span>
          </div>
        </div>

        <!-- PROGRESS BAR — only when uploading -->
        <div v-if="upload.status === 'uploading'" class="progress-wrap">
          <div class="progress-track">
            <div class="progress-bar" :style="{ width: upload.progress + '%' }"></div>
          </div>
          <span class="progress-text">{{ upload.progress }}%</span>
        </div>

        <!-- STATUS ROW -->
        <div class="status-row">
          <span class="status-badge" :class="'status-' + upload.status">
            {{ statusLabel(upload.status) }}
          </span>

          <!-- Completed: show checkmark + upload time -->
          <span v-if="upload.status === 'completed'" class="status-detail success-detail">
            ✅ Sent to server at {{ formatTime(upload.uploaded_at) }}
          </span>

          <!-- Pending: explain why waiting -->
          <span v-if="upload.status === 'pending'" class="status-detail pending-detail">
            Waiting for internet connection
          </span>

          <!-- Uploading: show live % -->
          <span v-if="upload.status === 'uploading'" class="status-detail uploading-detail">
            Uploading… {{ upload.progress }}% complete
          </span>

          <!-- Failed: show exact error -->
          <span v-if="upload.status === 'failed'" class="status-detail failed-detail">
            ❌ {{ shortError(upload.error_msg) }}
          </span>
        </div>

        <!-- FULL ERROR BOX — expanded error for failed -->
        <div v-if="upload.status === 'failed'" class="error-box">
          <p class="error-box-title">Error Details:</p>
          <p class="error-box-msg">{{ upload.error_msg || 'Unknown error' }}</p>
          <p class="error-box-hint">
            {{ errorHint(upload.error_msg) }}
          </p>
        </div>

        <!-- LOCAL FILE PATH -->
        <div class="path-row">
          <span class="detail-label">Local copy:</span>
          <span class="path-value">{{ upload.local_path }}</span>
        </div>

      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

defineProps({
  uploads: { type: Array, required: true },
})

const emit = defineEmits(['upload-deleted'])

const expandedId   = ref(null)
const chunkMap     = ref({})
const loadingChunks = ref(false)

function handleDelete(id) {
  emit('upload-deleted', id)
}

async function toggleChunks(uploadId) {
  if (expandedId.value === uploadId) {
    expandedId.value = null
    return
  }
  expandedId.value = uploadId
  loadingChunks.value = true
  try {
    const chunks = await invoke('get_chunks', { uploadId })
    chunkMap.value[uploadId] = chunks
  } catch (e) {
    console.error('Failed to load chunks:', e)
  } finally {
    loadingChunks.value = false
  }
}

function formatSize(bytes) {
  if (!bytes || bytes === 0) return '—'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function formatDate(dateStr) {
  if (!dateStr) return '—'
  return new Date(dateStr).toLocaleString()
}

function formatTime(dateStr) {
  if (!dateStr) return '—'
  return new Date(dateStr).toLocaleTimeString()
}

function statusLabel(status) {
  const labels = {
    pending:   '⏳ Pending',
    uploading: '⬆️ Uploading',
    completed: '✅ Completed',
    failed:    '❌ Failed',
  }
  return labels[status] || status
}

function chunkStatusLabel(status) {
  const labels = {
    pending:   '⏳',
    uploading: '⬆️',
    done:      '✅',
    failed:    '❌',
  }
  return labels[status] || status
}

function fileIcon(name) {
  if (!name) return '📄'
  const ext = name.split('.').pop().toLowerCase()
  const icons = {
    pdf: '📕', csv: '📊', xlsx: '📗', xls: '📗',
    png: '🖼️', jpg: '🖼️', jpeg: '🖼️',
    doc: '📘', docx: '📘', txt: '📄',
    zip: '🗜️', rar: '🗜️',
  }
  return icons[ext] || '📄'
}

function shortError(msg) {
  if (!msg) return 'Unknown error'
  if (msg.includes('connection refused')) return 'Server not reachable'
  if (msg.includes('error sending request')) return 'Network error'
  if (msg.includes('timeout')) return 'Connection timed out'
  return msg.slice(0, 60) + (msg.length > 60 ? '…' : '')
}

function errorHint(msg) {
  if (!msg) return ''
  if (msg.includes('connection refused') || msg.includes('error sending request')) {
    return '💡 Will retry automatically when internet is available.'
  }
  return '💡 Will retry when internet is available.'
}
</script>


<style scoped>
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-title {
  font-size: 15px;
  margin: 0;
  color: #2d2d2d;
}

.count-badge {
  font-size: 11px;
  background: #e8e8e8;
  color: #666;
  padding: 2px 8px;
  border-radius: 10px;
}

.empty-message { font-size: 13px; color: #888; }

.upload-list {
  list-style: none;
  margin: 0; padding: 0;
  display: flex; flex-direction: column; gap: 12px;
}

.upload-item {
  padding: 12px;
  border-radius: 8px;
  border: 1px solid #ececec;
  display: flex; flex-direction: column; gap: 8px;
}

.item-pending   { border-left: 3px solid #f0c040; background: #fffdf0; }
.item-uploading { border-left: 3px solid #2d6cdf; background: #f0f5ff; }
.item-completed { border-left: 3px solid #27ae60; background: #f0fff5; }
.item-failed    { border-left: 3px solid #e74c3c; background: #fff5f5; }

.upload-top {
  display: flex;
  align-items: center;
  gap: 10px;
}

.file-type-icon { font-size: 28px; flex-shrink: 0; }

.file-info { flex: 1; min-width: 0; }

.upload-filename {
  margin: 0;
  font-size: 13px; font-weight: bold; color: #2d2d2d;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}

.upload-id {
  margin: 2px 0 0;
  font-size: 10px; color: #bbb; font-family: monospace;
}

.delete-btn {
  background: none; border: 1px solid #ddd;
  border-radius: 50%; width: 24px; height: 24px;
  font-size: 10px; color: #aaa; cursor: pointer;
  display: flex; align-items: center; justify-content: center;
  flex-shrink: 0; transition: all 0.15s;
}
.delete-btn:hover { background: #fee; color: #e74c3c; border-color: #f1948a; }

.detail-row {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.detail-label {
  font-size: 10px;
  color: #aaa;
  text-transform: uppercase;
  letter-spacing: 0.4px;
}

.detail-value {
  font-size: 12px;
  color: #555;
}

.progress-wrap {
  display: flex;
  align-items: center;
  gap: 8px;
}

.progress-track {
  flex: 1;
  background: #dde;
  border-radius: 4px;
  height: 8px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #2d6cdf, #5b9cf6);
  border-radius: 4px;
  transition: width 0.4s ease;
}

.progress-text {
  font-size: 11px;
  color: #2d6cdf;
  font-weight: bold;
  width: 32px;
  text-align: right;
}

.status-row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.status-badge {
  font-size: 11px; font-weight: bold;
  padding: 3px 10px; border-radius: 10px;
  flex-shrink: 0;
}
.status-pending   { background: #fef9e7; color: #b7950b; }
.status-uploading { background: #eaf4fd; color: #2471a3; }
.status-completed { background: #eafaf1; color: #1e8449; }
.status-failed    { background: #fdedec; color: #c0392b; }

.status-detail {
  font-size: 12px;
}
.success-detail   { color: #1e8449; }
.pending-detail   { color: #b7950b; }
.uploading-detail { color: #2471a3; }
.failed-detail    { color: #c0392b; }

.error-box {
  background: #fff0f0;
  border: 1px solid #f1948a;
  border-radius: 6px;
  padding: 8px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.error-box-title {
  margin: 0;
  font-size: 11px;
  font-weight: bold;
  color: #c0392b;
  text-transform: uppercase;
  letter-spacing: 0.4px;
}

.error-box-msg {
  margin: 0;
  font-size: 11px;
  color: #555;
  font-family: monospace;
  word-break: break-all;
}

.error-box-hint {
  margin: 0;
  font-size: 11px;
  color: #888;
}

.path-row {
  display: flex;
  gap: 6px;
  align-items: flex-start;
  flex-wrap: wrap;
}

.path-value {
  font-size: 10px;
  color: #aaa;
  font-family: monospace;
  word-break: break-all;
}
.chunk-toggle { margin-top: 4px; }

.chunk-toggle-btn {
  background: none;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 11px;
  color: #666;
  cursor: pointer;
  padding: 3px 10px;
}
.chunk-toggle-btn:hover { background: #f5f5f5; }

.chunk-loading { font-size: 12px; color: #aaa; margin: 4px 0; }

.chunk-table-wrap { overflow-x: auto; }

.chunk-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
  margin-top: 4px;
}

.chunk-table th {
  text-align: left;
  padding: 4px 8px;
  background: #f5f5f5;
  color: #888;
  font-weight: 600;
  border-bottom: 1px solid #eee;
}

.chunk-table td {
  padding: 4px 8px;
  border-bottom: 1px solid #f5f5f5;
  color: #555;
}

.chunk-done      td { background: #f9fff9; }
.chunk-failed    td { background: #fff9f9; }
.chunk-uploading td { background: #f0f5ff; }
.chunk-pending   td { background: #fffef5; }

.chunk-badge {
  padding: 1px 6px;
  border-radius: 8px;
  font-size: 10px;
}
.cbadge-done      { background: #eafaf1; color: #1e8449; }
.cbadge-failed    { background: #fdedec; color: #c0392b; }
.cbadge-uploading { background: #eaf4fd; color: #2471a3; }
.cbadge-pending   { background: #fef9e7; color: #b7950b; }

.chunk-error { color: #e74c3c; max-width: 120px; word-break: break-all; }
</style>