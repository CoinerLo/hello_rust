<template>
    <div>
        <h1>Список чатов</h1>
    </div>
    <div>
        
    </div>
</template>

<script>
    import { mapActions } from "vuex";

    export default {
        name: 'ChatsPage',
        methods: {
            ...mapActions(['fetchChats']),
        },
        mounted() {
            this.fetchChats();
        },
    }
</script>

<style>
    
</style>
