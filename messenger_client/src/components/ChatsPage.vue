<template>
    <div>
        <h1>Список чатов</h1>
    </div>
    <div>
        <form @submit.prevent="createChat">
            <input v-model="chatName" placeholder="Название чата" required />
            <button>Создать чат</button>
        </form>
    </div>
    <div>
        <ol v-if="chats.length" class="chat-list">
            <li v-for="chat in chats" :key="chat.id" class="chat-item">
                <span>{{chat.name}}</span>
                <button @click="openChat(chat.id)">Перейти в чат</button>
                
            </li>
        </ol>
        <p v-else>Нет доступных чатов.</p>
    </div>
</template>

<script>
    import { mapActions, mapState } from "vuex";

    export default {
        name: 'ChatsPage',
        data() {
            return {
                username: '',
                chatName: '',
            };
        },
        computed: {
            ...mapState(['chats']),
        },
        methods: {
            ...mapActions(['fetchChats', 'createGroupChat']),
            async createChat() {
                try {
                    await this.createGroupChat(this.chatName);
                } catch (e) {
                    console.log(e);
                    alert('Ошибка создания чата');
                }
            },
        },
        mounted() {
            this.fetchChats();
        },
    }
</script>

<style scoped>
    .chat-list {
        max-width: 600px;
        margin: 0 auto;
    }

    .chat-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 10px;
        border-bottom: 1px solid #ccc;
    }

    .chat-item:last-child {
        border-bottom: none;
    }

    button {
        padding: 5px 10px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    button:hover {
        background-color: #0056b3;
    }
</style>
