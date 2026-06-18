<template>
  <div class="network-status" :class="isOnline ? 'status-online' : 'status-offline'">
    <span class="status-dot">{{ isOnline ? '🟢' : '🔴' }}</span>
    <span class="status-text">{{ isOnline ? 'Online' : 'Offline' }}</span>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

// navigator.onLine is a browser built-in property that returns true or false
// based on the network state at the moment the code runs.
// We use it to set the correct initial value when the component first loads —
// so if you open the app while already offline, it shows 🔴 immediately.
const isOnline = ref(navigator.onLine)

// These are the handler functions we'll attach to window events.
// They're defined as named functions (not inline arrows) so we can pass
// the exact same reference to both addEventListener and removeEventListener.
// If we used arrow functions inline in onMounted/onUnmounted, they'd be
// two different function objects and the removal would silently fail.
function handleOnline() {
  isOnline.value = true
}

function handleOffline() {
  isOnline.value = false
}

// onMounted runs once, after this component is added to the DOM.
// This is the right place to register event listeners because the
// component is fully ready and we know it exists.
onMounted(() => {
  window.addEventListener('online', handleOnline)
  window.addEventListener('offline', handleOffline)
})

// onUnmounted runs once, just before this component is removed from the DOM.
// We remove the same listeners we added in onMounted.
// Skipping this step would leave orphaned listeners in memory — they'd keep
// running and trying to update a component that no longer exists.
onUnmounted(() => {
  window.removeEventListener('online', handleOnline)
  window.removeEventListener('offline', handleOffline)
})
</script>

<style scoped>
.network-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  border-radius: 20px;
  font-size: 13px;
  font-weight: bold;
  /* Center the pill horizontally under the heading */
  width: fit-content;
  margin: 0 auto 8px auto;
  /* Smooth background color transition when status changes */
  transition: background-color 0.3s ease;
}

.status-online {
  background-color: #eafaf1;
  color: #1e8449;
  border: 1px solid #a9dfbf;
}

.status-offline {
  background-color: #fdedec;
  color: #c0392b;
  border: 1px solid #f1948a;
}

.status-dot {
  font-size: 12px;
  /* Prevent the emoji from being squished on narrow windows */
  flex-shrink: 0;
}

.status-text {
  letter-spacing: 0.3px;
}
</style>