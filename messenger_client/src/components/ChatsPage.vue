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
        
    </div>
</template>

<script>
    import { mapActions } from "vuex";

    export default {
        name: 'ChatsPage',
        data() {
            return {
                username: '',
                chatName: '',
            };
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

<style>
    
</style>
