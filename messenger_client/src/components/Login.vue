<template>
  <div>
    <h2>Авторизация</h2>
    <form @submit.prevent="login">
      <input v-model="username" placeholder="Имя пользователя" required />
      <input v-model="password" type="password" placeholder="Пароль" required />
      <button type="submit">Войти</button>
    </form>
  </div>
</template>

<script>
  import { mapActions } from 'vuex';

  
</script>
