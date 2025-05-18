import { createRouter, createWebHistory } from "vue-router";
import RegisterForm from "@/components/RegisterForm.vue";
import LoginPage from "@/components/LoginPage.vue";
import ChatPage from "@/components/ChatPage.vue";
import DefaultPage from "@/components/DefaultPage.vue";
import ChatsPage from "@/components/ChatsPage.vue";

const routes = [
  { path: '/', component: DefaultPage },
  { path: '/register', component: RegisterForm },
  { path: '/login', component: LoginPage },
  { path: '/chat/:chatId', component: ChatPage },
  { path: '/chat', component: ChatsPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
