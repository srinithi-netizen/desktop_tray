<template>
  <div class="activation-wrapper">
    <div class="activation-card">

      <div class="activation-header">
        <h1>FluxBooks</h1>
        <p class="activation-subtitle">Activate Your Account</p>
      </div>

      <!-- Success message -->
      <div v-if="isSuccess" class="alert alert-success">
        ✅ Account activated successfully. Redirecting…
      </div>

      <!-- Error message -->
      <div v-if="errorMessage" class="alert alert-error">
        ⚠️ {{ errorMessage }}
      </div>

      <div class="form-group">
        <label>Email Address</label>
        <input
          v-model="email"
          type="email"
          placeholder="you@example.com"
          :disabled="isLoading || isSuccess"
        />
      </div>

      <div class="form-group">
        <label>Activation Code</label>
        <input
          v-model="activationCode"
          type="text"
          placeholder="Enter code from your email"
          :disabled="isLoading || isSuccess"
        />
        <button class="resend-btn" @click="handleResend" :disabled="isLoading || isSuccess">
          Resend Code
        </button>
      </div>

      <div class="form-group">
        <label>New Password</label>
        <input
          v-model="password"
          type="password"
          placeholder="Min. 8 characters"
          :disabled="isLoading || isSuccess"
        />
      </div>

      <div class="form-group">
        <label>Confirm Password</label>
        <input
          v-model="confirmPassword"
          type="password"
          placeholder="Re-enter your password"
          :disabled="isLoading || isSuccess"
        />
      </div>

      <button
        class="activate-btn"
        @click="handleActivate"
        :disabled="isLoading || isSuccess"
      >
        {{ isLoading ? 'Activating…' : 'Activate Account' }}
      </button>

    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['activated'])

// Form fields
const email           = ref('')
const activationCode  = ref('')
const password        = ref('')
const confirmPassword = ref('')

// UI state
const isLoading    = ref(false)
const isSuccess    = ref(false)
const errorMessage = ref('')

// ── Mock data — change these to match your real API later ──
const MOCK_EMAIL          = 'user@fluxbooks.com'
const MOCK_CODE           = 'ABC123'
const MIN_PASSWORD_LENGTH = 8

function handleResend() {
  errorMessage.value = ''
  // In a real app this would call an API to resend the email
  alert(`Mock: Activation code sent to ${email.value || 'your email'}.\n\nFor this POC use code: ABC123`)
}

async function handleActivate() {
  errorMessage.value = ''

  // ── Validations ──────────────────────────────────────────

  if (!email.value) {
    errorMessage.value = 'Please enter your email address.'
    return
  }

  // Mock: check email exists
  if (email.value.toLowerCase() !== MOCK_EMAIL) {
    errorMessage.value = 'Email address is not registered.'
    return
  }

  if (!activationCode.value) {
    errorMessage.value = 'Please enter your activation code.'
    return
  }

  // Mock: check activation code
  if (activationCode.value.toUpperCase() !== MOCK_CODE) {
    errorMessage.value = 'Invalid activation code. Please check and try again.'
    return
  }

  if (!password.value) {
    errorMessage.value = 'Please enter a password.'
    return
  }

  if (password.value.length < MIN_PASSWORD_LENGTH) {
    errorMessage.value = `Password must be at least ${MIN_PASSWORD_LENGTH} characters.`
    return
  }

  if (password.value !== confirmPassword.value) {
    errorMessage.value = 'Password and Confirm Password do not match.'
    return
  }

  // ── Mock activation ──────────────────────────────────────

  isLoading.value = true

  // Simulate network delay
  await new Promise(resolve => setTimeout(resolve, 1200))

  // Save session to localStorage
  localStorage.setItem('fluxbooks_session', JSON.stringify({
    email: email.value,
    activatedAt: new Date().toISOString(),
  }))

  isLoading.value = false
  isSuccess.value = true

  // Wait 1.5s then tell App.vue to switch to main app
  setTimeout(() => {
    emit('activated')
  }, 1500)
}
</script>

<style scoped>
.activation-wrapper {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f4f5f7;
  padding: 24px;
}

.activation-card {
  background: white;
  border-radius: 12px;
  padding: 32px 28px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 2px 12px rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.activation-header {
  text-align: center;
  margin-bottom: 4px;
}

.activation-header h1 {
  font-size: 24px;
  color: #2d6cdf;
  margin: 0 0 4px 0;
}

.activation-subtitle {
  font-size: 14px;
  color: #888;
  margin: 0;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 13px;
  font-weight: 600;
  color: #444;
}

.form-group input {
  padding: 9px 12px;
  font-size: 14px;
  border: 1px solid #ddd;
  border-radius: 6px;
  outline: none;
  transition: border-color 0.15s;
}

.form-group input:focus {
  border-color: #2d6cdf;
}

.form-group input:disabled {
  background-color: #f9f9f9;
  color: #aaa;
}

.resend-btn {
  align-self: flex-start;
  background: none;
  border: none;
  font-size: 12px;
  color: #2d6cdf;
  cursor: pointer;
  padding: 0;
  text-decoration: underline;
}

.resend-btn:disabled {
  color: #aaa;
  cursor: default;
}

.activate-btn {
  padding: 10px;
  font-size: 14px;
  font-weight: 600;
  background-color: #2d6cdf;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s;
  margin-top: 4px;
}

.activate-btn:hover:not(:disabled) {
  background-color: #1f54b3;
}

.activate-btn:disabled {
  background-color: #a0b4e0;
  cursor: default;
}

.alert {
  padding: 10px 14px;
  border-radius: 6px;
  font-size: 13px;
}

.alert-success {
  background-color: #eafaf1;
  color: #1e8449;
  border: 1px solid #a9dfbf;
}

.alert-error {
  background-color: #fdedec;
  color: #c0392b;
  border: 1px solid #f1948a;
}
</style>