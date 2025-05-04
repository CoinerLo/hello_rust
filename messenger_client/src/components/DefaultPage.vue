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

<style scoped>
    .start-page {
        text-align: center;
        padding: 50px;
        font-family: Arial, sans-serif;
    }

    h1 {
        font-size: 2.5rem;
        margin-bottom: 20px;
    }

    p {
        font-size: 1.2rem;
        color: #555;
        margin-bottom: 30px;
    }

    .actions {
        display: flex;
        justify-content: center;
        gap: 20px;
    }

    .btn {
        padding: 10px 20px;
        font-size: 1rem;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .btn:hover {
        opacity: 0.9;
    }

    .btn:first-child {
        background-color: #4caf50; /* Зелёная кнопка для регистрации */
        color: white;
    }

    .btn:last-child {
        background-color: #2196f3; /* Синяя кнопка для входа */
        color: white;
    }
</style>
