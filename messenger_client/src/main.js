import { createApp } from 'vue'
import App from './App.vue'
import store from './store'
import router from './router'
import createWebSocketManager from './services/socket'

const app = createApp(App);
const webSocketManager = createWebSocketManager(store);

app.use(store);
app.use(router);

app.mount('#app');

export { webSocketManager };
