<template>
  <div class="card">
    <h2 class="section-title">Upload History</h2>

    <p v-if="uploads.length === 0" class="empty-message">
      No uploads yet
    </p>

    <ul v-else class="upload-list">
      <li
        v-for="upload in uploads"
        :key="upload.id"
        class="upload-item"
      >
        <span class="upload-filename">{{ upload.fileName }}</span>

        <span class="upload-meta">
          <span class="upload-status">{{ upload.status }}</span>
          <span class="upload-date">{{ upload.uploadedAt }}</span>
        </span>

        <!-- Delete button. @click passes the specific upload's id
             to handleDelete so we know exactly which record to remove. -->
        <button class="delete-btn" @click="handleDelete(upload.id)">
          Delete
        </button>
      </li>
    </ul>
  </div>
</template>

<script setup>
defineProps({
  uploads: {
    type: Array,
    required: true,
  },
})

// Declare the event this component is allowed to emit upward.
const emit = defineEmits(['upload-deleted'])

// Called when any Delete button is clicked.
// 'id' comes from the @click handler: @click="handleDelete(upload.id)"
function handleDelete(id) {
  // This component does not touch localStorage or modify any array.
  // It simply tells the parent (App.vue) which id the user wants removed.
  // App.vue owns the data and decides what to do with it.
  emit('upload-deleted', id)
}
</script>

<style scoped>
.section-title {
  font-size: 15px;
  margin: 0 0 12px 0;
  color: #2d2d2d;
}

.empty-message {
  font-size: 13px;
  color: #888;
}

.upload-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.upload-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background-color: #f9f9f9;
  border-radius: 6px;
  border: 1px solid #ececec;
}

.upload-filename {
  font-size: 13px;
  font-weight: bold;
  color: #2d2d2d;
  max-width: 160px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.upload-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 3px;
}

.upload-status {
  font-size: 12px;
  color: #27ae60;
  font-weight: bold;
}

.upload-date {
  font-size: 11px;
  color: #aaa;
}

.delete-btn {
  margin-left: 12px;
  padding: 5px 10px;
  font-size: 12px;
  background-color: #e74c3c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  /* flex-shrink: 0 prevents the button from being squished
     when the filename or date text is long. */
  flex-shrink: 0;
}

.delete-btn:hover {
  background-color: #c0392b;
}
</style>