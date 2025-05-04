<template>
  <div class="start-page">
    <h1>Добро пожаловать в чат</h1>
    <p>
      Это современный мессенджер, где вы можете общаться с друзьями, создавать групповые чаты и обмениваться сообщениями в реальном времени.
    </p>
    <div class="actions">
      <button @click="goToRegister" class="btn">Зарегистрироваться</button>
      <button @click="goToLogin" class="btn">Войти</button>
    </div>
  </div>
</template>

<script>
  export default {
    methods: {
      goToRegister() {
        this.$router.push('/register');
      },
      goToLogin() {
        this.$router.push('/login');
      },
    },
  }
</script>
