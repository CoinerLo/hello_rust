<template>
  <div>
    <h2>Чат</h2>
    <div v-for="message in messages" :key="message.id">
      <strong>{{ message.sender }}</strong>{{ message.content }}
    </div>
    <form @submit.prevent="sendMessage">
      <input v-model="newMessage" placeholder="Введите сообщение" required />
      <button type="submit">Отправить</button>
    </form>
  </div>
</template>

<script>
  import { mapState, mapActions } from 'vuex';

  export default {
    data() {
      return {
        newMessage: '',
      }
    },
    computed: {
      ...mapState(['messages']),
    },
    methods: {
      ...mapActions(['sendMessage']),
      async sendMessage() {
        if (this.newMessage.trim()) {
          await this.sendMessage({
            chatId: this.$route.params.chatId,
            content: this.newMessage,
          });
          this.newMessage = '';
        }
      },
    },
  };
</script>
