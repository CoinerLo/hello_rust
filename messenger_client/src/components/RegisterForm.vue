<template>
  <div>
    <h2>Регистрация</h2>
    <form @submit.prevent="register">
      <input v-model="username" placeholder="Имя пользователя" required />
      <input v-model="password" type="password" placeholder="Пароль" required />
      <button>Зарегистрироваться</button>
    </form>
  </div>
</template>

<script>
  import api from "@/services/api"

  export default {
    data() {
      return {
        username: '',
        password: '',
      };
    },
    methods: {
      async register() {
        try {
          await api.register(this.username, this.password);
          this.$router.push('/login');
        } catch (e) {
          alert('Ошибка регистрации');
        }
      },
    },
  };
</script>
