<template>
  <div>
    <h2>Авторизация</h2>
    <form @submit.prevent="userLogin">
      <input v-model="username" placeholder="Имя пользователя" required />
      <input v-model="password" type="password" placeholder="Пароль" required />
      <button type="submit">Войти</button>
    </form>
  </div>
</template>

<script>
  import { mapActions } from 'vuex';
  import { webSocketManager } from '../main';

  export default {
    data: function() {
      return {
        username: '',
        password: '',
      };
    },
    methods: {
      ...mapActions(['login']),
      async userLogin() {
        try {
          await this.login({ username: this.username, password: this.password });
          webSocketManager.connect(this.username);
          this.$router.push('/chat');
        } catch (e) {
          alert('Ошибка авторизации')
        }
      },
    }
  };
</script>
