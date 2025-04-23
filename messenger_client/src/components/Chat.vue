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


