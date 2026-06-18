<template>
  <div class="card">
    <label for="client-select" class="field-label">Client</label>
    <select id="client-select" v-model="selectedClient" class="select-input">
      <option disabled value="">-- Select a client --</option>
      <option v-for="client in clients" :key="client" :value="client">
        {{ client }}
      </option>
    </select>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// Hardcoded list of clients for now.
// Later this could come from localStorage or be entered by the user.
const clients = ref([
  'Acme Restaurant',
  'Blue Bottle Retail',
  'Sunrise Wellness Spa'
])

// Holds whichever client the user currently has selected.
const selectedClient = ref('')
</script>

<style scoped>
.field-label {
  display: block;
  font-size: 13px;
  font-weight: bold;
  margin-bottom: 6px;
  
  color: #555;
}

.select-input {
  width: 100%;
  padding: 8px;
  font-size: 14px;
  border-radius: 4px;
  border: 1px solid #ccc;
}
</style>
