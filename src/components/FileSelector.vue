<template>
  <div class="card">
    <button class="select-btn" @click="handleSelectFile">
      Select File
    </button>

    <p class="filename-display">
      {{ selectedFileName || 'No file selected yet' }}
    </p>

    <!-- Upload button only appears once a file has been selected. -->
    <button
      v-if="selectedFileName"
      class="upload-btn"
      @click="handleUpload"
    >
      Upload
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

// defineEmits declares the custom events this component can fire upward.
// App.vue listens for 'upload-saved'.
const emit = defineEmits(['upload-saved'])

const selectedFileName = ref('')

async function handleSelectFile() {
  const filePath = await open({
    multiple: false,
    directory: false,
  })

  if (filePath) {
    selectedFileName.value = filePath.split(/[\\/]/).pop()
  }
}

function handleUpload() {
  // Build the upload record. This is the object that gets stored
  // in localStorage and displayed in the history list.
  const newUpload = {
    id: Date.now(),                          // unique ID — milliseconds since epoch
    fileName: selectedFileName.value,
    status: 'Uploaded',
    uploadedAt: new Date().toLocaleString(), // human-readable timestamp
  }

  // Read the current array out of localStorage.
  // If nothing is there yet, default to an empty array.
  const existing = JSON.parse(localStorage.getItem('uploads') || '[]')

  // Add the new record to the front so newest appears first in the list.
  existing.unshift(newUpload)

  // Write the updated array back to localStorage as a JSON string.
  localStorage.setItem('uploads', JSON.stringify(existing))

  // Tell App.vue about the new record so it can update its reactive
  // array without needing to re-read localStorage.
  emit('upload-saved', newUpload)

  // Clear the selected filename so the user starts fresh for the next file.
  selectedFileName.value = ''
}
</script>

<style scoped>
.select-btn {
  padding: 8px 16px;
  font-size: 14px;
  background-color: #2d6cdf;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.select-btn:hover {
  background-color: #1f54b3;
}

.filename-display {
  margin-top: 10px;
  font-size: 13px;
  color: #666;
}

.upload-btn {
  margin-top: 12px;
  padding: 8px 16px;
  font-size: 14px;
  background-color: #27ae60;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: block;
}

.upload-btn:hover {
  background-color: #1e8449;
}
</style>