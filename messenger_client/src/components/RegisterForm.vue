<template>
  <div>
    <h2>Регистрация</h2>
    <form @submit.prevent="userRegister">
      <input v-model="username" placeholder="Имя пользователя" required />
      <input v-model="password" type="password" placeholder="Пароль" required />
      <button type="submit">Зарегистрироваться</button>
    </form>
  </div>
</template>

<script>
  import { mapActions } from 'vuex';

  export default {
    data() {
      return {
        username: '',
        password: '',
      };
    },
    methods: {
      ...mapActions(['register']),
      async userRegister() {
        try {
          await this.register({ username: this.username, password: this.password });
          this.$router.push('/login');
        } catch (e) {
          alert('Ошибка регистрации');
        }
      },
    },
  };
</script>
